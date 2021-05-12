// This file is @generated by dependabot-config-internal-codegen.
// It is not intended for manual editing.

use std::fmt;

use crate::*;
impl fmt::Display for v1::PackageManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Javascript => f.write_str("javascript"),
            Self::RubyBundler => f.write_str("ruby:bundler"),
            Self::PhpComposer => f.write_str("php:composer"),
            Self::Python => f.write_str("python"),
            Self::GoModules => f.write_str("go:modules"),
            Self::GoDep => f.write_str("go:dep"),
            Self::JavaMaven => f.write_str("java:maven"),
            Self::JavaGradle => f.write_str("java:gradle"),
            Self::DotnetNuget => f.write_str("dotnet:nuget"),
            Self::RustCargo => f.write_str("rust:cargo"),
            Self::ElixirHex => f.write_str("elixir:hex"),
            Self::Docker => f.write_str("docker"),
            Self::Terraform => f.write_str("terraform"),
            Self::Submodules => f.write_str("submodules"),
            Self::Elm => f.write_str("elm"),
        }
    }
}
impl fmt::Display for v1::UpdateSchedule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Live => f.write_str("live"),
            Self::Daily => f.write_str("daily"),
            Self::Weekly => f.write_str("weekly"),
            Self::Monthly => f.write_str("monthly"),
        }
    }
}
impl fmt::Display for v1::AllowedDependencyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Development => f.write_str("development"),
            Self::Production => f.write_str("production"),
            Self::Direct => f.write_str("direct"),
            Self::Indirect => f.write_str("indirect"),
            Self::All => f.write_str("all"),
        }
    }
}
impl fmt::Display for v1::AllowedUpdateType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Security => f.write_str("security"),
            Self::All => f.write_str("all"),
        }
    }
}
impl fmt::Display for v1::AutomergedDependencyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Development => f.write_str("development"),
            Self::Production => f.write_str("production"),
            Self::All => f.write_str("all"),
        }
    }
}
impl fmt::Display for v1::AutomergedUpdateType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SecurityPatch => f.write_str("security:patch"),
            Self::SemverPatch => f.write_str("semver:patch"),
            Self::SemverMinor => f.write_str("semver:minor"),
            Self::InRange => f.write_str("in_range"),
            Self::All => f.write_str("all"),
        }
    }
}
impl fmt::Display for v1::VersionRequirementUpdate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Off => f.write_str("off"),
            Self::Auto => f.write_str("auto"),
            Self::WidenRanges => f.write_str("widen_ranges"),
            Self::IncreaseVersions => f.write_str("increase_versions"),
            Self::IncreaseVersionsIfNecessary => f.write_str("increase_versions_if_necessary"),
        }
    }
}
impl fmt::Display for v2::PackageEcosystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bundler => f.write_str("bundler"),
            Self::Cargo => f.write_str("cargo"),
            Self::Composer => f.write_str("composer"),
            Self::Docker => f.write_str("docker"),
            Self::Hex => f.write_str("hex"),
            Self::Elm => f.write_str("elm"),
            Self::Gitsubmodule => f.write_str("gitsubmodule"),
            Self::GithubActions => f.write_str("github-actions"),
            Self::Gomod => f.write_str("gomod"),
            Self::Gradle => f.write_str("gradle"),
            Self::Maven => f.write_str("maven"),
            Self::Npm => f.write_str("npm"),
            Self::Nuget => f.write_str("nuget"),
            Self::Pip => f.write_str("pip"),
            Self::Terraform => f.write_str("terraform"),
        }
    }
}
impl fmt::Display for v2::Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Daily => f.write_str("daily"),
            Self::Weekly => f.write_str("weekly"),
            Self::Monthly => f.write_str("monthly"),
        }
    }
}
impl fmt::Display for v2::Day {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Monday => f.write_str("monday"),
            Self::Tuesday => f.write_str("tuesday"),
            Self::Wednesday => f.write_str("wednesday"),
            Self::Thursday => f.write_str("thursday"),
            Self::Friday => f.write_str("friday"),
            Self::Saturday => f.write_str("saturday"),
            Self::Sunday => f.write_str("sunday"),
        }
    }
}
impl fmt::Display for v2::DependencyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Direct => f.write_str("direct"),
            Self::Indirect => f.write_str("indirect"),
            Self::All => f.write_str("all"),
            Self::Production => f.write_str("production"),
            Self::Development => f.write_str("development"),
        }
    }
}
impl fmt::Display for v2::CommitMessageInclude {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Scope => f.write_str("scope"),
        }
    }
}
impl fmt::Display for v2::InsecureExternalCodeExecution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Allow => f.write_str("allow"),
            Self::Deny => f.write_str("deny"),
        }
    }
}
impl fmt::Display for v2::RebaseStrategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Disabled => f.write_str("disabled"),
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl fmt::Display for v2::VersioningStrategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LockfileOnly => f.write_str("lockfile-only"),
            Self::Auto => f.write_str("auto"),
            Self::Widen => f.write_str("widen"),
            Self::Increase => f.write_str("increase"),
            Self::IncreaseIfNecessary => f.write_str("increase-if-necessary"),
        }
    }
}
impl fmt::Display for v2::RegistryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ComposerRepository => f.write_str("composer-repository"),
            Self::DockerRegistry => f.write_str("docker-registry"),
            Self::Git => f.write_str("git"),
            Self::MavenRepository => f.write_str("maven-repository"),
            Self::NpmRegistry => f.write_str("npm-registry"),
            Self::NugetFeed => f.write_str("nuget-feed"),
            Self::PythonIndex => f.write_str("python-index"),
            Self::RubygemsServer => f.write_str("rubygems-server"),
        }
    }
}