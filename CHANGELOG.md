# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.4] - 2026-10-11

- Allow dirty Cargo.lock during crates.io publish so CI can ship README/metadata fixes.

## [1.0.3] - 2026-10-11

- Republish after Cargo.lock sync so crates.io publish succeeds.

## [1.0.2] - 2026-10-11

- Fixed crates.io/docs.rs README badges and install snippets to use crate name `licensechain`.
- Aligned Rust version badge and docs with MSRV 1.85; documentation URL https://docs.licensechain.app/en/sdk.

## [1.0.1] - 2026-10-10

- Relicensed public surface under Elastic License 2.0 (ELv2).
- Migrated crate edition from 2021 to 2024; declared MSRV `rust-version = "1.85"`.
- Added Dependabot for Cargo and GitHub Actions.

## [1.0.0] - 2026-04-06

- Initial public SDK surface with repository documentation kit.
