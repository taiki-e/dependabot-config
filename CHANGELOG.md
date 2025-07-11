# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org).

Releases may yanked if there is a security bug, a soundness bug, or a regression.

<!--
Note: In this file, do not use the hard wrap in the middle of a sentence for compatibility with GitHub comment style markdown rendering.
-->

## [Unreleased]

- Rename `v2::PackageEcosystem::Hex` to `v2::PackageEcosystem::Mix` (Hex is the package manager). ([#15](https://github.com/taiki-e/dependabot-config/pull/15), thanks @reneleonhardt)

- Make `v2::Update::directory` field optional to reflect upstream change due to addition of `v2::Update::directories`. ([377dec7](https://github.com/taiki-e/dependabot-config/commit/377dec785615e90f2340b2afb9ab5748f21211c6))

- Make `v2::Registry::url` field optional. ([377dec7](https://github.com/taiki-e/dependabot-config/commit/377dec785615e90f2340b2afb9ab5748f21211c6))

- Change `v2::Registry::type_` field to use `v2::RegistryType` instead of `String`. ([377dec7](https://github.com/taiki-e/dependabot-config/commit/377dec785615e90f2340b2afb9ab5748f21211c6))

- Change `v2::Update::{milestone,open_pull_requests_limit}` and `v1::UpdateConfig::default_milestone` fields to use `u32` instead of `usize`. ([95d63f1](https://github.com/taiki-e/dependabot-config/commit/95d63f1cd755353bcea5d8eac5ef4a376bb2acbf))

- Add `v2::PackageEcosystem::{Bun,Devcontainers,DockerCompose,DotnetSdk,Helm,Uv}` variants. ([#15](https://github.com/taiki-e/dependabot-config/pull/15), thanks @reneleonhardt)

- Add `v2::Update::directories` field. ([377dec7](https://github.com/taiki-e/dependabot-config/commit/377dec785615e90f2340b2afb9ab5748f21211c6))

- Documentation improvements.

## [0.3.4] - 2024-04-13

- Add `#[must_use]` to constructors.

## [0.3.3] - 2023-10-18

- Improve compile time.

## [0.3.2] - 2023-08-12

This release updates v2 types based on the latest GitHub docs.

- Add `v2::Dependabot::enable_beta_ecosystems` field.

- Add `v2::PackageEcosystem::{Pub,Swift}` variants.

- Add `v2::Ignore::update_types` field and `v2::UpdateType` enum.

- Add `v2::Registry::key` field.

- Add `v2::RegistryType::{HexOrganization,TerraformRegistry}` variants.

- Documentation improvements.

## [0.3.1] - 2023-07-03

- Fix build error from dependency when built with `-Z minimal-versions`.

## [0.3.0] - 2023-06-26

- Update `indexmap` to 2.0.

  **Note:** This raises the minimum supported Rust version of this crate from Rust 1.58 to Rust 1.64.

## [0.2.2] - 2022-07-29

- Update `serde_yaml` to 0.9.

  **Note:** This raises the minimum supported Rust version of this crate from Rust 1.56 to Rust 1.58.

## [0.2.1] - 2022-04-26

- Implement `Hash` for `v2::PackageEcosystem`. ([#8](https://github.com/taiki-e/dependabot-config/pull/8), thanks @andreimoustache)

- Implement `Hash` for `v1::PackageManager`.

## [0.2.0] - 2021-04-17

- [Support Dependabot v1 configuration file.](https://github.com/taiki-e/dependabot-config/pull/3)

- [Add `v2::CommitMessageInclude` and `v2::InsecureExternalCodeExecution`.](https://github.com/taiki-e/dependabot-config/pull/3)

- [Change `v2::CommitMessage::include` field from `Option<String>` to `Option<v2::CommitMessageInclude>`.](https://github.com/taiki-e/dependabot-config/pull/3)

- [Change `v2::Update::insecure_external_code_execution` field from `Option<String>` to `Option<v2::InsecureExternalCodeExecution>`.](https://github.com/taiki-e/dependabot-config/pull/3)

- [Implement `Display` for `v2::{PackageEcosystem, Interval, Day, DependencyType, RebaseStrategy, VersioningStrategy, RegistryType, Separator}`.](https://github.com/taiki-e/dependabot-config/pull/3)

## [0.1.0] - 2021-04-09

Initial release

[Unreleased]: https://github.com/taiki-e/dependabot-config/compare/v0.3.4...HEAD
[0.3.4]: https://github.com/taiki-e/dependabot-config/compare/v0.3.3...v0.3.4
[0.3.3]: https://github.com/taiki-e/dependabot-config/compare/v0.3.2...v0.3.3
[0.3.2]: https://github.com/taiki-e/dependabot-config/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/taiki-e/dependabot-config/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/taiki-e/dependabot-config/compare/v0.2.2...v0.3.0
[0.2.2]: https://github.com/taiki-e/dependabot-config/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/taiki-e/dependabot-config/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/taiki-e/dependabot-config/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/taiki-e/dependabot-config/releases/tag/v0.1.0
