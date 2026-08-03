//! RS256 license_token verification via JWKS (parity with Node `verifyLicenseAssertionJwt`).

use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use rsa::pkcs1::{EncodeRsaPublicKey, LineEnding};
use rsa::{BigUint, RsaPublicKey};
use serde_json::Value;

/// Must match Core API `LICENSE_TOKEN_USE_CLAIM`.
pub const LICENSE_TOKEN_USE_CLAIM: &str = "licensechain_license_v1";

#[derive(Debug, Clone, Default)]
pub struct VerifyLicenseAssertionOptions {
    pub expected_app_id: Option<String>,
    pub issuer: Option<String>,
}

fn b64url_decode(s: &str) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
    use base64::engine::general_purpose::URL_SAFE_NO_PAD;
    use base64::Engine;
    Ok(URL_SAFE_NO_PAD.decode(s.trim())?)
}

fn jwk_rsa_decoding_key(
    jwks_json: &Value,
    kid: Option<&str>,
) -> Result<DecodingKey, Box<dyn std::error::Error + Send + Sync>> {
    let keys = jwks_json
        .get("keys")
        .and_then(|k| k.as_array())
        .ok_or("JWKS missing keys array")?;
    for key in keys {
        if key.get("kty").and_then(|v| v.as_str()) != Some("RSA") {
            continue;
        }
        if let Some(want) = kid {
            let key_kid = key.get("kid").and_then(|v| v.as_str()).unwrap_or("");
            if key_kid != want {
                continue;
            }
        }
        let n_b64 = key
            .get("n")
            .and_then(|v| v.as_str())
            .ok_or("JWK missing n")?;
        let e_b64 = key
            .get("e")
            .and_then(|v| v.as_str())
            .ok_or("JWK missing e")?;
        let n_bytes = b64url_decode(n_b64)?;
        let e_bytes = b64url_decode(e_b64)?;
        let n = BigUint::from_bytes_be(&n_bytes);
        let e = BigUint::from_bytes_be(&e_bytes);
        let public_key = RsaPublicKey::new(n, e)?;
        let pem = public_key.to_pkcs1_pem(LineEnding::LF)?;
        return Ok(DecodingKey::from_rsa_pem(pem.as_bytes())?);
    }
    Err("no matching RSA JWK".into())
}

fn aud_matches(claims: &Value, expected: &str) -> bool {
    match claims.get("aud") {
        Some(Value::String(s)) => s == expected,
        Some(Value::Array(arr)) => arr.iter().filter_map(|v| v.as_str()).any(|s| s == expected),
        _ => false,
    }
}

/// Verify a `license_token` using JWKS (RS256). Fetches `jwks_url` with `http` client.
pub async fn verify_license_assertion_jwt(
    http: &reqwest::Client,
    token: &str,
    jwks_url: &str,
    opts: Option<VerifyLicenseAssertionOptions>,
) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    let token = token.trim();
    if token.is_empty() {
        return Err("empty token".into());
    }
    let jwks_url = jwks_url.trim();
    if jwks_url.is_empty() {
        return Err("empty jwks_url".into());
    }

    let header = decode_header(token)?;
    let kid = header.kid.as_deref();

    let jwks_resp = http.get(jwks_url).send().await?;
    if !jwks_resp.status().is_success() {
        return Err(format!("JWKS HTTP {}", jwks_resp.status()).into());
    }
    let jwks_json: Value = jwks_resp.json().await?;
    let key = jwk_rsa_decoding_key(&jwks_json, kid)?;

    let mut validation = Validation::new(Algorithm::RS256);
    if let Some(iss) = opts
        .as_ref()
        .and_then(|o| o.issuer.as_deref())
        .filter(|s| !s.is_empty())
    {
        validation.set_issuer(&[iss]);
    }

    let token_data = decode::<Value>(token, &key, &validation)?;
    let claims = token_data.claims;

    let tu = claims
        .get("token_use")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    if tu != LICENSE_TOKEN_USE_CLAIM {
        return Err(format!("token_use: want {:?}", LICENSE_TOKEN_USE_CLAIM).into());
    }

    if let Some(exp) = opts.as_ref().and_then(|o| o.expected_app_id.as_deref()) {
        let exp = exp.trim();
        if !exp.is_empty() && !aud_matches(&claims, exp) {
            return Err("aud mismatch".into());
        }
    }

    Ok(claims)
}
