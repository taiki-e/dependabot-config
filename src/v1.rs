// SPDX-License-Identifier: Apache-2.0 OR MIT

//! The Dependabot v1 configuration.
//!
//! See [Dependabot Docs][docs] for more.
//!
//! [docs]: https://dependabot.com/docs/config-file

#![allow(missing_docs)]

use serde::de::{self, Deserialize, Deserializer};
use serde_derive::{Deserialize, Serialize};

/// The Dependabot v1 configuration.
///
/// See [Dependabot Docs][docs] for more.
///
/// [docs]: https://dependabot.com/docs/config-file
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Dependabot {
    #[serde(deserialize_with = "de_version")]
    version: u8,
    /// Configuration options for updates
    pub update_configs: Vec<UpdateConfig>,
}

impl Dependabot {
    /// Creates a new `Dependabot`.
    #[must_use]
    pub fn new(update_configs: Vec<UpdateConfig>) -> Self {
        Self { version: 1, update_configs }
    }
}

impl Default for Dependabot {
    fn default() -> Self {
        Self::new(vec![])
    }
}

#[allow(clippy::to_string_trait_impl)]
impl ToString for Dependabot {
    fn to_string(&self) -> String {
        serde_yaml::to_string(&self).unwrap()
    }
}

fn de_version<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    let n: u8 = Deserialize::deserialize(deserializer)?;
    match n {
        1 => Ok(n),
        _ => Err(de::Error::custom(format!(
            "The property 'version' value \"{n}\" did not match: 1",
        ))),
    }
}

/// Configuration option for updates
///
/// See [Dependabot Docs][docs] for more.
///
/// [docs]: https://dependabot.com/docs/config-file/#available-configuration-options
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct UpdateConfig {
    /// What package manager to use.
    pub package_manager: PackageManager,
    /// Where to look for package manifests.
    ///
    /// See [Dependabot Docs][docs] for more.
    ///
    /// [docs]: https://dependabot.com/docs/config-file/#directory-required
    pub directory: String,
    /// How often to check for updates.
    pub update_schedule: UpdateSchedule,
    /// Branch to create pull requests against.
    ///
    /// See [Dependabot Docs][docs] for more.
    ///
    /// [docs]: https://dependabot.com/docs/config-file/#target_branch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_branch: Option<String>,
    /// Reviewers to set on pull requests.
    ///
    /// See [Dependabot Docs][docs] for more.
    ///
    /// [docs]: https://dependabot.com/docs/config-file/#default_reviewers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_reviewers: Option<Vec<String>>,
    /// Assignees to set on pull requests.
    ///
    /// See [Dependabot Docs][docs] for more.
    ///
    /// [docs]: https://dependabot.com/docs/config-file/#default_assignees
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_assignees: Option<Vec<String>>,
    /// Labels to set on pull requests.
    ///
    /// See [Dependabot Docs][docs] for more.
    ///
    /// [docs]: https://dependabot.com/docs/config-file/#default_labels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_labels: Option<Vec<String>>,
    /// Milestone to set on pull requests.
    ///
    /// See [Dependabot Docs][docs] for more.
    ///
    /// [docs]: https://dependabot.com/docs/config-file/#default_milestone
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_milestone: Option<u32>,
    /// Limit which updates are allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<AllowedUpdate>>,
    /// Ignore certain dependencies or versions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignored_updates: Option<Vec<IgnoredUpdate>>,
    /// Updates that should be merged automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automerged_updates: Option<Vec<AutomergedUpdate>>,
    /// How to update manifest version requirements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_requirement_updates: Option<VersionRequirementUpdate>,
    /// Commit message preferences.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_message: Option<CommitMessage>,
}

impl UpdateConfig {
    /// Creates a new `UpdateConfig`.
    #[must_use]
    pub fn new<D: Into<String>>(
        package_manager: PackageManager,
        directory: D,
        update_schedule: UpdateSchedule,
    ) -> Self {
        Self {
            package_manager,
            directory: directory.into(),
            update_schedule,
            target_branch: None,
            default_reviewers: None,
            default_assignees: None,
            default_labels: None,
            default_milestone: None,
            allowed_updates: None,
            ignored_updates: None,
            automerged_updates: None,
            version_requirement_updates: None,
            commit_message: None,
        }
    }
}

/// Package manager to use.
///
/// See [Dependabot Docs][docs] for more.
///
/// [docs]: https://dependabot.com/docs/config-file/#package_manager-required
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum PackageManager {
    /// `javascript`
    Javascript,
    /// `ruby:bundler`
    #[serde(rename = "ruby:bundler")]
    RubyBundler,
    /// `php:composer`
    #[serde(rename = "php:composer")]
    PhpComposer,
    /// `python`
    Python,
    /// `go:modules`
    #[serde(rename = "go:modules")]
    GoModules,
    /// `go:dep`
    #[serde(rename = "go:dep")]
    GoDep,
    /// `java:maven`
    #[serde(rename = "java:maven")]
    JavaMaven,
    /// `java:gradle`
    #[serde(rename = "java:gradle")]
    JavaGradle,
    /// `elixir:hex`
    #[serde(rename = "dotnet:nuget")]
    DotnetNuget,
    /// `rust:cargo`
    #[serde(rename = "rust:cargo")]
    RustCargo,
    /// `elixir:hex`
    #[serde(rename = "elixir:hex")]
    ElixirHex,
    /// `docker`
    Docker,
    /// `terraform`
    Terraform,
    /// `submodules`
    Submodules,
    /// `elm`
    Elm,
}

/// How often to check for updates.
///
/// See [Dependabot Docs][docs] for more.
///
/// [docs]: https://dependabot.com/docs/config-file/#update_schedule-required
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum UpdateSchedule {
    Live,
    /// Runs on every weekday, Monday to Friday.
    Daily,
    /// Runs once each week.
    Weekly,
    /// Runs once each month.
    Monthly,
}

/// Customize which updates are allowed.
///
/// See [Dependabot Docs][docs] for more.
///
/// [docs]: https://dependabot.com/docs/config-file/#allowed_updates
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct AllowedUpdate {
    #[serde(rename = "match")]
    pub match_: AllowedUpdateMatch,
}

/// Customize which updates are allowed.
///
/// See [Dependabot Docs][docs] for more.
///
/// [docs]: https://dependabot.com/docs/config-file/#allowed_updates
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct AllowedUpdateMatch {
    /// Allow updates for dependencies with matching names, optionally using * to match zero or more characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependency_name: Option<String>,
    /// Allow updates for dependencies of specific types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependency_type: Option<AllowedDependencyType>,
    pub update_type: Option<AllowedUpdateType>,
}

/// Allow updates for dependencies of specific types.
///
/// See [Dependabot Docs][docs] for more.
///
/// [docs]: https://dependabot.com/docs/config-file/#allowed_updates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum AllowedDependencyType {
    /// Development dependency group (supported by some package managers).
    Development,
    /// Production dependency group (supported by some package managers).
    Production,
    /// Direct/top-level dependencies.
    Direct,
    /// Indirect/transient/sub-dependencies.
    Indirect,
    All,
}

/// Allowed update type.
///
/// See [Dependabot Docs][docs] for more.
///
/// [docs]: https://dependabot.com/docs/config-file/#allowed_updates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum AllowedUpdateType {
    Security,
    All,
}

/// Ignore certain dependencies or versions.
///
/// See [Dependabot Docs][docs] for more.
///
/// [docs]: https://dependabot.com/docs/config-file/#ignored_updates
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct IgnoredUpdate {
    #[serde(rename = "match")]
    pub match_: IgnoredUpdateMatch,
}

/// Ignore certain dependencies or versions.
///
/// See [Dependabot Docs][docs] for more.
///
/// [docs]: https://dependabot.com/docs/config-file/#ignored_updates
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct IgnoredUpdateMatch {
    /// Ignore updates for dependencies with matching names, optionally using * to match zero or more characters.
    pub dependency_name: String,
    /// Ignore specific versions or ranges of versions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_requirement: Option<String>,
}

impl IgnoredUpdate {
    /// Creates a new `Ignore`.
    #[must_use]
    pub fn new(dependency_name: String) -> Self {
        Self { match_: IgnoredUpdateMatch { dependency_name, version_requirement: None } }
    }
}

/// Update that should be merged automatically.
///
/// See [Dependabot Docs][docs] for more.
///
/// [docs]: https://dependabot.com/docs/config-file/#automerged_updates
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct AutomergedUpdate {
    #[serde(rename = "match")]
    pub match_: AutomergedUpdateMatch,
}

/// Update that should be merged automatically.
///
/// See [Dependabot Docs][docs] for more.
///
/// [docs]: https://dependabot.com/docs/config-file/#automerged_updates
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct AutomergedUpdateMatch {
    pub dependency_name: Option<String>,
    pub dependency_type: Option<AutomergedDependencyType>,
    pub update_type: Option<AutomergedUpdateType>,
}

/// Dependency types that should be merged automatically.
///
/// See [Dependabot Docs][docs] for more.
///
/// [docs]: https://dependabot.com/docs/config-file/#automerged_updates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum AutomergedDependencyType {
    Development,
    Production,
    All,
}

/// Update types that should be merged automatically.
///
/// See [Dependabot Docs][docs] for more.
///
/// [docs]: https://dependabot.com/docs/config-file/#automerged_updates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum AutomergedUpdateType {
    /// SemVer patch update that fixes a known security vulnerability.
    #[serde(rename = "security:patch")]
    SecurityPatch,
    /// SemVer patch update, e.g. > 1.x && 1.0.1 to 1.0.3.
    #[serde(rename = "semver:patch")]
    SemverPatch,
    /// SemVer minor update, e.g. > 1.x && 2.1.4 to 2.3.1.
    #[serde(rename = "semver:minor")]
    SemverMinor,
    /// Matching the version requirement in your package manifest.
    InRange,
    All,
}

/// How to update manifest version requirements.
///
/// See [Dependabot Docs][docs] for more.
///
/// [docs]: https://dependabot.com/docs/config-file/#version_requirement_updates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum VersionRequirementUpdate {
    /// Only lockfile updates, ignoring updates that require package manifest changes.
    Off,
    /// Increase versions if an app, widen ranges if a library.
    Auto,
    /// Relax the version requirement to include both the new and old version when possible.
    WidenRanges,
    /// Always increase the version requirement to match the new version.
    IncreaseVersions,
    /// Increase the version requirement only when required by the new version.
    IncreaseVersionsIfNecessary,
}

/// Commit message preferences.
///
/// See [Dependabot Docs][docs] for more.
///
/// [docs]: https://dependabot.com/docs/config-file/#commit_message
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct CommitMessage {
    /// Specify a prefix for all commit messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// Specify a separate prefix for all commit messages that update dependencies in the Development dependency group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_development: Option<String>,
    /// Specify that any prefix is followed by a list of the dependencies updated in the commit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_scope: Option<bool>,
}
