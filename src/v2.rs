// SPDX-License-Identifier: Apache-2.0 OR MIT

//! The Dependabot v2 configuration.
//!
//! See [GitHub Docs][docs] for more.
//!
//! [docs]: https://docs.github.com/en/code-security/dependabot/working-with-dependabot/dependabot-options-reference

// TODO: Update based on the latest docs
// TODO: add groups once stabilized (currently in beta): https://docs.github.com/en/code-security/dependabot/dependabot-version-updates/configuration-options-for-the-dependabot.yml-file#groups

use core::fmt;

use indexmap::IndexMap;
use serde::de::{self, Deserialize, Deserializer};
use serde_derive::{Deserialize, Serialize};

/// The Dependabot v2 configuration.
///
/// See [GitHub Docs][docs] for more.
///
/// [docs]: https://docs.github.com/en/code-security/dependabot/working-with-dependabot/dependabot-options-reference
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Dependabot {
    #[serde(deserialize_with = "de_version")]
    version: u8,
    /// Opt in to updates for ecosystems that are not yet generally available.
    ///
    /// See [GitHub Docs][docs] for more.
    ///
    /// [docs]: https://docs.github.com/en/code-security/dependabot/working-with-dependabot/dependabot-options-reference#enable-beta-ecosystems-
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_beta_ecosystems: Option<bool>,
    /// Configuration options for private registries.
    #[serde(default, skip_serializing_if = "Registries::is_empty")]
    pub registries: Registries,
    /// Configuration options for updates
    pub updates: Vec<Update>,
}

impl Dependabot {
    /// Creates a new `Dependabot`.
    #[must_use]
    pub fn new(updates: Vec<Update>) -> Self {
        Self {
            version: 2,
            enable_beta_ecosystems: None,
            registries: Registries::default(),
            updates,
        }
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
        2 => Ok(n),
        _ => Err(de::Error::custom(format!(
            "The property 'version' value \"{n}\" did not match: 2",
        ))),
    }
}

/// Configuration option for updates
///
/// See [GitHub Docs][docs] for more.
///
/// [docs]: https://docs.github.com/en/code-security/dependabot/working-with-dependabot/dependabot-options-reference#required-keys
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Update {
    /// Package manager to use.
    pub package_ecosystem: PackageEcosystem,
    /// Location of package manifests.
    ///
    /// See [GitHub Docs][docs] for more.
    ///
    /// [docs]: https://docs.github.com/en/code-security/dependabot/working-with-dependabot/dependabot-options-reference#directories-or-directory--
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory: Option<String>,
    /// Locations of package manifests.
    ///
    /// See [GitHub Docs][docs] for more.
    ///
    /// [docs]: https://docs.github.com/en/code-security/dependabot/working-with-dependabot/dependabot-options-reference#directories-or-directory--
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directories: Option<Vec<String>>,
    /// How often to check for updates.
    pub schedule: Schedule,
    /// Customize which updates are allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow: Option<Vec<Allow>>,
    /// Assignees to set on pull requests.
    ///
    /// See [GitHub Docs][docs] for more.
    ///
    /// [docs]: https://docs.github.com/en/code-security/dependabot/working-with-dependabot/dependabot-options-reference#assignees--
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignees: Option<Vec<String>>,
    /// Commit message preferences.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_message: Option<CommitMessage>,
    /// Ignore certain dependencies or versions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore: Option<Vec<Ignore>>,
    /// Allow or deny code execution in manifest files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insecure_external_code_execution: Option<InsecureExternalCodeExecution>,
    /// Labels to set on pull requests.
    ///
    /// See [GitHub Docs][docs] for more.
    ///
    /// [docs]: https://docs.github.com/en/code-security/dependabot/working-with-dependabot/dependabot-options-reference#labels--
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// Milestone to set on pull requests.
    ///
    /// See [GitHub Docs][docs] for more.
    ///
    /// [docs]: https://docs.github.com/en/code-security/dependabot/working-with-dependabot/dependabot-options-reference#milestone--
    #[serde(skip_serializing_if = "Option::is_none")]
    pub milestone: Option<u32>,
    /// Limit number of open pull requests for version updates.
    ///
    /// See [GitHub Docs][docs] for more.
    ///
    /// [docs]: https://docs.github.com/en/code-security/dependabot/working-with-dependabot/dependabot-options-reference#open-pull-requests-limit-
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_pull_requests_limit: Option<u32>,
    /// Change separator for pull request branch names.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_branch_name: Option<PullRequestBranchName>,
    /// Disable automatic rebasing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rebase_strategy: Option<RebaseStrategy>,
    /// Reviewers to set on pull requests.
    ///
    /// See [GitHub Docs][docs] for more.
    ///
    /// [docs]: https://docs.github.com/en/code-security/dependabot/working-with-dependabot/dependabot-options-reference#reviewers--
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviewers: Option<Vec<String>>,
    /// Branch to create pull requests against.
    ///
    /// See [GitHub Docs][docs] for more.
    ///
    /// [docs]: https://docs.github.com/en/code-security/dependabot/working-with-dependabot/dependabot-options-reference#target-branch-
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_branch: Option<String>,
    /// Update vendored or cached dependencies.
    ///
    /// See [GitHub Docs][docs] for more.
    ///
    /// [docs]: https://docs.github.com/en/code-security/dependabot/working-with-dependabot/dependabot-options-reference#vendor--
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<bool>,
    /// How to update manifest version requirements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioning_strategy: Option<VersioningStrategy>,
}

impl Update {
    /// Creates a new `Update`.
    #[must_use]
    pub fn new<D: Into<String>>(
        package_ecosystem: PackageEcosystem,
        directory: D,
        schedule: Schedule,
    ) -> Self {
        Self {
            package_ecosystem,
            directory: Some(directory.into()),
            directories: None,
            schedule,
            allow: None,
            assignees: None,
            commit_message: None,
            ignore: None,
            insecure_external_code_execution: None,
            labels: None,
            milestone: None,
            open_pull_requests_limit: None,
            pull_request_branch_name: None,
            rebase_strategy: None,
            reviewers: None,
            target_branch: None,
            vendor: None,
            versioning_strategy: None,
        }
    }
}

/// Package manager to use.
///
/// See [GitHub Docs][docs] for more.
///
/// [docs]: https://docs.github.com/en/code-security/dependabot/working-with-dependabot/dependabot-options-reference#package-ecosystem-
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub enum PackageEcosystem {
    /// `bun`
    Bun,
    /// `bundler`
    Bundler,
    /// `cargo`
    Cargo,
    /// `composer`
    Composer,
    /// `devcontainers`
    Devcontainers,
    /// `docker`
    Docker,
    /// `docker-compose`
    DockerCompose,
    /// `dotnet-sdk`
    DotnetSdk,
    /// `mix` (package manager "Hex")
    Mix,
    /// `helm`
    Helm,
    /// `elm`
    Elm,
    /// `gitsubmodule`
    Gitsubmodule,
    /// `github-actions`
    GithubActions,
    /// `gomod`
    Gomod,
    /// `gradle`
    Gradle,
    /// `maven`
    Maven,
    /// `npm` (package manager "npm", "pnpm", or "yarn")
    Npm,
    /// `nuget`
    Nuget,
    /// `pip` (package manager "pip", "pip-compile", "pipenv", or "poetry")
    Pip,
    /// `pub`
    Pub,
    /// `swift`
    Swift,
    /// `terraform`
    Terraform,
    /// `uv`
    Uv,
}

/// How often to check for updates.
///
/// See [GitHub Docs][docs] for more.
///
/// [docs]: https://docs.github.com/en/code-security/dependabot/working-with-dependabot/dependabot-options-reference#schedule-
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Schedule {
    /// How often to check for updates.
    pub interval: Interval,
    /// Specify an alternative day to check for updates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day: Option<Day>,
    /// Specify an alternative time of day to check for updates.
    ///
    /// See [GitHub Docs][docs] for more.
    ///
    /// [docs]: https://docs.github.com/en/code-security/dependabot/working-with-dependabot/dependabot-options-reference#schedule-
    // TODO: Should we wrap this in its own type and verify the content?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    /// Specify an alternative time zone.
    ///
    /// See [GitHub Docs][docs] for more.
    ///
    /// [docs]: https://docs.github.com/en/code-security/dependabot/working-with-dependabot/dependabot-options-reference#schedule-
    // TODO: Should we wrap this in its own type and verify the content?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

impl Schedule {
    /// Creates a new `Schedule`.
    #[must_use]
    pub fn new(interval: Interval) -> Self {
        Self { interval, day: None, time: None, timezone: None }
    }
}

/// How often to check for updates.
///
/// See [GitHub Docs][docs] for more.
///
/// [docs]: https://docs.github.com/en/code-security/dependabot/working-with-dependabot/dependabot-options-reference#schedule-
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub enum Interval {
    /// Runs on every weekday, Monday to Friday.
    Daily,
    /// Runs once each week. By default, this is on Monday.
    Weekly,
    /// Runs once each month. This is on the first day of the month.
    Monthly,
}

/// Specify an alternative day to check for updates.
///
/// See [GitHub Docs][docs] for more.
///
/// [docs]: https://docs.github.com/en/code-security/dependabot/working-with-dependabot/dependabot-options-reference#schedule-
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub enum Day {
    /// Monday.
    Monday,
    /// Tuesday.
    Tuesday,
    /// Wednesday.
    Wednesday,
    /// Thursday.
    Thursday,
    /// Friday.
    Friday,
    /// Saturday.
    Saturday,
    /// Sunday.
    Sunday,
}

impl Default for Day {
    fn default() -> Self {
        Self::Monday
    }
}

/// Customize which updates are allowed.
///
/// See [GitHub Docs][docs] for more.
///
/// [docs]: https://docs.github.com/en/code-security/dependabot/working-with-dependabot/dependabot-options-reference#allow--
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Allow {
    /// Allow updates for dependencies with matching names, optionally using * to match zero or more characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependency_name: Option<String>,
    /// Allow updates for dependencies of specific types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependency_type: Option<DependencyType>,
}

/// Allow updates for dependencies of specific types.
///
/// See [GitHub Docs][docs] for more.
///
/// [docs]: https://docs.github.com/en/code-security/dependabot/working-with-dependabot/dependabot-options-reference#allow--
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub enum DependencyType {
    /// All explicitly defined dependencies.
    Direct,
    /// Dependencies of direct dependencies (also known as sub-dependencies, or transient dependencies).
    Indirect,
    /// All explicitly defined dependencies. For bundler, pip, composer, cargo, also the dependencies of direct dependencies.
    All,
    /// Only dependencies in the "Product dependency group".
    Production,
    /// Only dependencies in the "Development dependency group".
    Development,
}

/// Commit message preferences.
///
/// See [GitHub Docs][docs] for more.
///
/// [docs]: https://docs.github.com/en/code-security/dependabot/working-with-dependabot/dependabot-options-reference#commit-message--
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
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
    pub include: Option<CommitMessageInclude>,
}

/// Specify that any prefix is followed by a list of the dependencies updated in the commit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub enum CommitMessageInclude {
    /// Specify that any prefix is followed by a list of the dependencies updated in the commit.
    Scope,
}

/// Ignore certain dependencies or versions.
///
/// See [GitHub Docs][docs] for more.
///
/// [docs]: https://docs.github.com/en/code-security/dependabot/working-with-dependabot/dependabot-options-reference#ignore--
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Ignore {
    /// Ignore updates for dependencies with matching names, optionally using * to match zero or more characters.
    pub dependency_name: String,
    /// Ignore specific versions or ranges of versions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<String>>,
    /// Ignore types of updates, such as semver major, minor, or patch updates on version updates.
    pub update_types: Option<Vec<UpdateType>>,
}

impl Ignore {
    /// Creates a new `Ignore`.
    #[must_use]
    pub fn new(dependency_name: String) -> Self {
        Self { dependency_name, versions: None, update_types: None }
    }
}

/// Types of updates, such as semver major, minor, or patch updates.
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub enum UpdateType {
    /// `version-update:semver-major`
    #[serde(rename = "version-update:semver-major")]
    SemverMajor,
    /// `version-update:semver-minor`
    #[serde(rename = "version-update:semver-minor")]
    SemverMinor,
    /// `version-update:semver-patch`
    #[serde(rename = "version-update:semver-patch")]
    SemverPatch,
}

/// Allow or deny code execution in manifest files.
///
/// See [GitHub Docs][docs] for more.
///
/// [docs]: https://docs.github.com/en/code-security/dependabot/working-with-dependabot/dependabot-options-reference#insecure-external-code-execution--
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub enum InsecureExternalCodeExecution {
    /// Allow external code execution.
    Allow,
    /// Explicitly deny external code execution, irrespective of whether there is a registries setting for this update configuration.
    Deny,
}

/// Change separator for pull request branch names.
///
/// See [GitHub Docs][docs] for more.
///
/// [docs]: https://docs.github.com/en/code-security/dependabot/working-with-dependabot/dependabot-options-reference#pull-request-branch-nameseparator--
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct PullRequestBranchName {
    /// Change separator for pull request branch names.
    pub separator: Separator,
}

/// Change separator for pull request branch names.
///
/// See [GitHub Docs][docs] for more.
///
/// [docs]: https://docs.github.com/en/code-security/dependabot/working-with-dependabot/dependabot-options-reference#pull-request-branch-nameseparator--
// Do not implement Copy because strings may be allowed in the future.
#[derive(Debug, Clone, Serialize)]
#[serde(transparent)]
pub struct Separator {
    repr: char,
}

impl fmt::Display for Separator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.repr, f)
    }
}

impl<'de> Deserialize<'de> for Separator {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let c: char = Deserialize::deserialize(deserializer)?;
        match c {
            '-' | '/' | '_' => Ok(Self { repr: c }),
            _ => Err(de::Error::custom(format!(
                "The property 'pull-request-branch-name/separator' value \"{c}\" \
                     did not match one of the following values: -, /, _",
            ))),
        }
    }
}

/// Disable automatic rebasing.
///
/// See [GitHub Docs][docs] for more.
///
/// [docs]: https://docs.github.com/en/code-security/dependabot/working-with-dependabot/dependabot-options-reference#rebase-strategy--
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub enum RebaseStrategy {
    /// Disable automatic rebasing.
    Disabled,
    /// Use the default behavior and rebase open pull requests when conflicts are detected.
    Auto,
}

/// How to update manifest version requirements.
///
/// See [GitHub Docs][docs] for more.
///
/// [docs]: https://docs.github.com/en/code-security/dependabot/working-with-dependabot/dependabot-options-reference#versioning-strategy--
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub enum VersioningStrategy {
    /// Follow the default strategy described above.
    Auto,
    /// Always increase the version requirement to match the new version.
    Increase,
    /// Increase the version requirement only when required by the new version.
    IncreaseIfNecessary,
    /// Only create pull requests to update lockfiles. Ignore any new versions that would require package manifest changes.
    LockfileOnly,
    /// Relax the version requirement to include both the new and old version, when possible.
    Widen,
}

/// Configuration options for private registries.
///
/// See [GitHub Docs][docs] for more.
///
/// [docs]: https://docs.github.com/en/code-security/dependabot/working-with-dependabot/dependabot-options-reference#registries--
pub type Registries = IndexMap<String, Registry>;

/// Configuration options for a private registry.
///
/// See [GitHub Docs][docs] for more.
///
/// [docs]: https://docs.github.com/en/code-security/dependabot/working-with-dependabot/configuring-access-to-private-registries-for-dependabot#configuring-private-registries
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Registry {
    /// Identifies the type of registry.
    #[serde(rename = "type")]
    pub type_: RegistryType,
    /// The URL to use to access the dependencies in this registry. The protocol is optional. If not specified, https:// is assumed. Dependabot adds or ignores trailing slashes as required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The username that Dependabot uses to access the registry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// A reference to a Dependabot secret containing the password for the specified user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// A reference to a Dependabot secret containing an access key for this registry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// A reference to a Dependabot secret containing an access token for this registry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// For registries, if the boolean value is true, Dependabot will resolve dependencies by using the specified URL rather than the base URL of that specific ecosystem.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaces_base: Option<bool>,
}

/// Identifies the type of registry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub enum RegistryType {
    /// The `cargo-registry` type.
    ///
    /// See [GitHub Docs][docs] for more.
    ///
    /// [docs]: https://docs.github.com/en/code-security/dependabot/working-with-dependabot/configuring-access-to-private-registries-for-dependabot#cargo-registry
    CargoRepository,
    /// The `composer-repository` type.
    ///
    /// See [GitHub Docs][docs] for more.
    ///
    /// [docs]: https://docs.github.com/en/code-security/dependabot/working-with-dependabot/configuring-access-to-private-registries-for-dependabot#composer-repository
    ComposerRepository,
    /// The `docker-registry` type.
    ///
    /// See [GitHub Docs][docs] for more.
    ///
    /// [docs]: https://docs.github.com/en/code-security/dependabot/working-with-dependabot/configuring-access-to-private-registries-for-dependabot#docker-registry
    DockerRegistry,
    /// The `git` type.
    ///
    /// See [GitHub Docs][docs] for more.
    ///
    /// [docs]: https://docs.github.com/en/code-security/dependabot/working-with-dependabot/configuring-access-to-private-registries-for-dependabot#git
    Git,
    /// The `hex-organization` type.
    ///
    /// See [GitHub Docs][docs] for more.
    ///
    /// [docs]: https://docs.github.com/en/code-security/dependabot/working-with-dependabot/configuring-access-to-private-registries-for-dependabot#hex-organization
    HexOrganization,
    // TODO: this type has some additional fields.
    // /// The `hex-repository` type.
    // ///
    // /// See [GitHub Docs][docs] for more.
    // ///
    // /// [docs]: https://docs.github.com/en/code-security/dependabot/working-with-dependabot/configuring-access-to-private-registries-for-dependabot#hex-repository
    // HexRepository,
    /// The `maven-repository` type.
    ///
    /// See [GitHub Docs][docs] for more.
    ///
    /// [docs]: https://docs.github.com/en/code-security/dependabot/working-with-dependabot/configuring-access-to-private-registries-for-dependabot#maven-repository
    MavenRepository,
    /// The `npm-registry` type.
    ///
    /// See [GitHub Docs][docs] for more.
    ///
    /// [docs]: https://docs.github.com/en/code-security/dependabot/working-with-dependabot/configuring-access-to-private-registries-for-dependabot#npm-registry
    NpmRegistry,
    /// The `nuget-feed` type
    ///
    /// See [GitHub Docs][docs] for more.
    ///
    /// [docs]: https://docs.github.com/en/code-security/dependabot/working-with-dependabot/configuring-access-to-private-registries-for-dependabot#nuget-feed
    NugetFeed,
    /// The `pub-repository` type.
    ///
    /// See [GitHub Docs][docs] for more.
    ///
    /// [docs]: https://docs.github.com/en/code-security/dependabot/working-with-dependabot/configuring-access-to-private-registries-for-dependabot#pub-repository
    PubRepository,
    /// The `python-index` type.
    ///
    /// See [GitHub Docs][docs] for more.
    ///
    /// [docs]: https://docs.github.com/en/code-security/dependabot/working-with-dependabot/configuring-access-to-private-registries-for-dependabot#python-index
    PythonIndex,
    /// The `rubygems-server` type.
    ///
    /// See [GitHub Docs][docs] for more.
    ///
    /// [docs]: https://docs.github.com/en/code-security/dependabot/working-with-dependabot/configuring-access-to-private-registries-for-dependabot#rubygems-server
    RubygemsServer,
    /// The `terraform-registry` type.
    ///
    /// See [GitHub Docs][docs] for more.
    ///
    /// [docs]: https://docs.github.com/en/code-security/dependabot/working-with-dependabot/configuring-access-to-private-registries-for-dependabot#terraform-registry
    TerraformRegistry,
}
