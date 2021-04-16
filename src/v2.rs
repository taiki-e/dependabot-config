//! The Dependabot v2 configuration.
//!
//! See [GitHub Docs][docs] for more.
//!
//! [docs]: https://docs.github.com/en/code-security/supply-chain-security/configuration-options-for-dependency-updates

use indexmap::IndexMap;
use serde::{de, Deserialize, Serialize};

/// The Dependabot v2 configuration.
///
/// See [GitHub Docs][docs] for more.
///
/// [docs]: https://docs.github.com/en/code-security/supply-chain-security/configuration-options-for-dependency-updates
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct Dependabot {
    #[serde(deserialize_with = "de_version")]
    version: u8,
    /// Configuration options for private registries.
    #[serde(default, skip_serializing_if = "Registries::is_empty")]
    pub registries: Registries,
    /// Configuration options for updates
    pub updates: Vec<Update>,
}

impl Dependabot {
    /// Creates a new `Dependabot`.
    pub fn new(updates: Vec<Update>) -> Self {
        Self { version: 2, registries: Registries::default(), updates }
    }
}

impl Default for Dependabot {
    fn default() -> Self {
        Self::new(vec![])
    }
}

impl ToString for Dependabot {
    fn to_string(&self) -> String {
        serde_yaml::to_string(&self).unwrap()
    }
}

#[allow(single_use_lifetimes)]
fn de_version<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: de::Deserializer<'de>,
{
    let n: u8 = Deserialize::deserialize(deserializer)?;
    match n {
        2 => Ok(n),
        _ => Err(de::Error::custom(format!(
            "The property 'version' value \"{}\" did not match: 2",
            n
        ))),
    }
}

/// Configuration option for updates
///
/// See [GitHub Docs][docs] for more.
///
/// [docs]: https://docs.github.com/en/code-security/supply-chain-security/configuration-options-for-dependency-updates#configuration-options-for-updates
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct Update {
    /// Package manager to use.
    pub package_ecosystem: PackageEcosystem,
    /// Location of package manifests.
    ///
    /// See [GitHub Docs][docs] for more.
    ///
    /// [docs]: https://docs.github.com/en/code-security/supply-chain-security/configuration-options-for-dependency-updates#directory
    pub directory: String,
    /// How often to check for updates.
    pub schedule: Schedule,
    /// Customize which updates are allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow: Option<Vec<Allow>>,
    /// Assignees to set on pull requests.
    ///
    /// See [GitHub Docs][docs] for more.
    ///
    /// [docs]: https://docs.github.com/en/code-security/supply-chain-security/configuration-options-for-dependency-updates#assignees
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
    /// [docs]: https://docs.github.com/en/code-security/supply-chain-security/configuration-options-for-dependency-updates#labels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// Milestone to set on pull requests.
    ///
    /// See [GitHub Docs][docs] for more.
    ///
    /// [docs]: https://docs.github.com/en/code-security/supply-chain-security/configuration-options-for-dependency-updates#milestone
    #[serde(skip_serializing_if = "Option::is_none")]
    pub milestone: Option<usize>,
    /// Limit number of open pull requests for version updates.
    ///
    /// See [GitHub Docs][docs] for more.
    ///
    /// [docs]: https://docs.github.com/en/code-security/supply-chain-security/configuration-options-for-dependency-updates#open-pull-requests-limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_pull_requests_limit: Option<usize>,
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
    /// [docs]: https://docs.github.com/en/code-security/supply-chain-security/configuration-options-for-dependency-updates#reviewers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviewers: Option<Vec<String>>,
    /// Branch to create pull requests against.
    ///
    /// See [GitHub Docs][docs] for more.
    ///
    /// [docs]: https://docs.github.com/en/code-security/supply-chain-security/configuration-options-for-dependency-updates#target-branch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_branch: Option<String>,
    /// Update vendored or cached dependencies.
    ///
    /// See [GitHub Docs][docs] for more.
    ///
    /// [docs]: https://docs.github.com/en/code-security/supply-chain-security/configuration-options-for-dependency-updates#vendor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<bool>,
    /// How to update manifest version requirements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioning_strategy: Option<VersioningStrategy>,
}

impl Update {
    /// Creates a new `Update`.
    pub fn new(
        package_ecosystem: PackageEcosystem,
        directory: impl Into<String>,
        schedule: Schedule,
    ) -> Self {
        Self {
            package_ecosystem,
            directory: directory.into(),
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
/// [docs]: https://docs.github.com/en/code-security/supply-chain-security/configuration-options-for-dependency-updates#package-ecosystem
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub enum PackageEcosystem {
    /// `bundler`
    Bundler,
    /// `cargo`
    Cargo,
    /// `composer`
    Composer,
    /// `docker`
    Docker,
    /// `hex`
    Hex,
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
    /// `npm`
    Npm,
    /// `nuget`
    Nuget,
    /// `pip`
    Pip,
    /// `terraform`
    Terraform,
}

/// How often to check for updates.
///
/// See [GitHub Docs][docs] for more.
///
/// [docs]: https://docs.github.com/en/code-security/supply-chain-security/configuration-options-for-dependency-updates#scheduleinterval
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
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
    /// [docs]: https://docs.github.com/en/code-security/supply-chain-security/configuration-options-for-dependency-updates#scheduletime
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    /// Specify an alternative time zone.
    ///
    /// See [GitHub Docs][docs] for more.
    ///
    /// [docs]: https://docs.github.com/en/code-security/supply-chain-security/configuration-options-for-dependency-updates#scheduletimezone
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

impl Schedule {
    /// Creates a new `Schedule`.
    pub fn new(interval: Interval) -> Self {
        Self { interval, day: None, time: None, timezone: None }
    }
}

/// How often to check for updates.
///
/// See [GitHub Docs][docs] for more.
///
/// [docs]: https://docs.github.com/en/code-security/supply-chain-security/configuration-options-for-dependency-updates#scheduleinterval
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
/// [docs]: https://docs.github.com/en/code-security/supply-chain-security/configuration-options-for-dependency-updates#scheduleday
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
/// [docs]: https://docs.github.com/en/code-security/supply-chain-security/configuration-options-for-dependency-updates#allow
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
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
/// [docs]: https://docs.github.com/en/code-security/supply-chain-security/configuration-options-for-dependency-updates#allow
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
/// [docs]: https://docs.github.com/en/code-security/supply-chain-security/configuration-options-for-dependency-updates#commit-message
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
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
/// [docs]: https://docs.github.com/en/code-security/supply-chain-security/configuration-options-for-dependency-updates#ignore
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct Ignore {
    /// Ignore updates for dependencies with matching names, optionally using * to match zero or more characters.
    pub dependency_name: String,
    /// Ignore specific versions or ranges of versions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<String>>,
}

impl Ignore {
    /// Creates a new `Ignore`.
    pub fn new(dependency_name: String) -> Self {
        Self { dependency_name, versions: None }
    }
}

/// Allow or deny code execution in manifest files.
///
/// See [GitHub Docs][docs] for more.
///
/// [docs]: https://docs.github.com/en/code-security/supply-chain-security/configuration-options-for-dependency-updates#insecure-external-code-execution
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
/// [docs]: https://docs.github.com/en/code-security/supply-chain-security/configuration-options-for-dependency-updates#pull-request-branch-nameseparator
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct PullRequestBranchName {
    /// Change separator for pull request branch names.
    pub separator: Separator,
}

/// Change separator for pull request branch names.
///
/// See [GitHub Docs][docs] for more.
///
/// [docs]: https://docs.github.com/en/code-security/supply-chain-security/configuration-options-for-dependency-updates#pull-request-branch-nameseparator
#[derive(Debug, Clone, Serialize)]
#[serde(transparent)]
// Do not implement Copy because strings may be allowed in the future.
#[allow(missing_copy_implementations)]
pub struct Separator {
    repr: char,
}

impl<'de> Deserialize<'de> for Separator {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let c: char = Deserialize::deserialize(deserializer)?;
        match c {
            '-' | '/' | '_' => Ok(Self { repr: c }),
            _ => Err(de::Error::custom(format!(
                "The property 'pull-request-branch-name/separator' value \"{}\" \
                     did not match one of the following values: -, /, _",
                c
            ))),
        }
    }
}

/// Disable automatic rebasing.
///
/// See [GitHub Docs][docs] for more.
///
/// [docs]: https://docs.github.com/en/code-security/supply-chain-security/configuration-options-for-dependency-updates#rebase-strategy
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
/// [docs]: https://docs.github.com/en/code-security/supply-chain-security/configuration-options-for-dependency-updates#versioning-strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub enum VersioningStrategy {
    /// Only create pull requests to update lockfiles. Ignore any new versions that would require package manifest changes.
    LockfileOnly,
    /// Follow the default strategy described above.
    Auto,
    /// Relax the version requirement to include both the new and old version, when possible.
    Widen,
    /// Always increase the version requirement to match the new version.
    Increase,
    /// Increase the version requirement only when required by the new version.
    IncreaseIfNecessary,
}

/// Configuration options for private registries.
///
/// See [GitHub Docs][docs] for more.
///
/// [docs]: https://docs.github.com/en/code-security/supply-chain-security/configuration-options-for-dependency-updates#configuration-options-for-private-registries
pub type Registries = IndexMap<String, Registry>;

/// Configuration options for private registry.
///
/// See [GitHub Docs][docs] for more.
///
/// [docs]: https://docs.github.com/en/code-security/supply-chain-security/configuration-options-for-dependency-updates#configuration-options-for-private-registries
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct Registry {
    /// Identifies the type of registry.
    #[serde(rename = "type")]
    pub type_: String,
    /// The URL to use to access the dependencies in this registry. The protocol is optional. If not specified, https:// is assumed. Dependabot adds or ignores trailing slashes as required.
    pub url: String,
    /// The username that Dependabot uses to access the registry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// A reference to a Dependabot secret containing the password for the specified user. For more information, see "Managing encrypted secrets for Dependabot."
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// A reference to a Dependabot secret containing an access token for this registry. For more information, see "Managing encrypted secrets for Dependabot."
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// For registries with type: python-index, if the boolean value is true, pip resolves dependencies by using the specified URL rather than the base URL of the Python Package Index (by default <https://pypi.org/simple>).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaces_base: Option<bool>,
}

/// Identifies the type of registry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub enum RegistryType {
    /// The `composer-repository` type.
    ///
    /// See [GitHub Docs][docs] for more.
    ///
    /// [docs]: https://docs.github.com/en/code-security/supply-chain-security/configuration-options-for-dependency-updates#composer-repository
    ComposerRepository,
    /// The `docker-registry` type.
    ///
    /// See [GitHub Docs][docs] for more.
    ///
    /// [docs]: https://docs.github.com/en/code-security/supply-chain-security/configuration-options-for-dependency-updates#docker-registry
    DockerRegistry,
    /// The git type.
    ///
    /// See [GitHub Docs][docs] for more.
    ///
    /// [docs]: https://docs.github.com/en/code-security/supply-chain-security/configuration-options-for-dependency-updates#git
    Git,
    /// The `maven-repository` type.
    ///
    /// See [GitHub Docs][docs] for more.
    ///
    /// [docs]: https://docs.github.com/en/code-security/supply-chain-security/configuration-options-for-dependency-updates#maven-repository
    MavenRepository,
    /// The `npm-registry` type.
    ///
    /// See [GitHub Docs][docs] for more.
    ///
    /// [docs]: https://docs.github.com/en/code-security/supply-chain-security/configuration-options-for-dependency-updates#npm-registry
    NpmRegistry,
    /// The `nuget-feed` type
    ///
    /// See [GitHub Docs][docs] for more.
    ///
    /// [docs]: https://docs.github.com/en/code-security/supply-chain-security/configuration-options-for-dependency-updates#nuget-feed
    NugetFeed,
    /// The `python-index` type.
    ///
    /// See [GitHub Docs][docs] for more.
    ///
    /// [docs]: https://docs.github.com/en/code-security/supply-chain-security/configuration-options-for-dependency-updates#python-index
    PythonIndex,
    /// The `rubygems-server` type.
    ///
    /// See [GitHub Docs][docs] for more.
    ///
    /// [docs]: https://docs.github.com/en/code-security/supply-chain-security/configuration-options-for-dependency-updates#rubygems-server
    RubygemsServer,
}
