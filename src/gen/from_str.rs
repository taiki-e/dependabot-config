// This file is @generated by dependabot-config-internal-codegen.
// It is not intended for manual editing.

#![cfg_attr(rustfmt, rustfmt::skip)]
#![allow(clippy::wildcard_imports)]

use core::str::FromStr;
use crate::*;
impl FromStr for Dependabot {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(s).map_err(Error::new)
    }
}
impl FromStr for v1::Dependabot {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(s).map_err(Error::new)
    }
}
impl FromStr for v1::UpdateConfig {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(s).map_err(Error::new)
    }
}
impl FromStr for v1::PackageManager {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(s).map_err(Error::new)
    }
}
impl FromStr for v1::UpdateSchedule {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(s).map_err(Error::new)
    }
}
impl FromStr for v1::AllowedUpdate {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(s).map_err(Error::new)
    }
}
impl FromStr for v1::AllowedUpdateMatch {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(s).map_err(Error::new)
    }
}
impl FromStr for v1::AllowedDependencyType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(s).map_err(Error::new)
    }
}
impl FromStr for v1::AllowedUpdateType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(s).map_err(Error::new)
    }
}
impl FromStr for v1::IgnoredUpdate {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(s).map_err(Error::new)
    }
}
impl FromStr for v1::IgnoredUpdateMatch {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(s).map_err(Error::new)
    }
}
impl FromStr for v1::AutomergedUpdate {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(s).map_err(Error::new)
    }
}
impl FromStr for v1::AutomergedUpdateMatch {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(s).map_err(Error::new)
    }
}
impl FromStr for v1::AutomergedDependencyType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(s).map_err(Error::new)
    }
}
impl FromStr for v1::AutomergedUpdateType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(s).map_err(Error::new)
    }
}
impl FromStr for v1::VersionRequirementUpdate {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(s).map_err(Error::new)
    }
}
impl FromStr for v1::CommitMessage {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(s).map_err(Error::new)
    }
}
impl FromStr for v2::Dependabot {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(s).map_err(Error::new)
    }
}
impl FromStr for v2::Update {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(s).map_err(Error::new)
    }
}
impl FromStr for v2::PackageEcosystem {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(s).map_err(Error::new)
    }
}
impl FromStr for v2::Schedule {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(s).map_err(Error::new)
    }
}
impl FromStr for v2::Interval {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(s).map_err(Error::new)
    }
}
impl FromStr for v2::Day {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(s).map_err(Error::new)
    }
}
impl FromStr for v2::Allow {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(s).map_err(Error::new)
    }
}
impl FromStr for v2::DependencyType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(s).map_err(Error::new)
    }
}
impl FromStr for v2::CommitMessage {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(s).map_err(Error::new)
    }
}
impl FromStr for v2::CommitMessageInclude {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(s).map_err(Error::new)
    }
}
impl FromStr for v2::Ignore {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(s).map_err(Error::new)
    }
}
impl FromStr for v2::InsecureExternalCodeExecution {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(s).map_err(Error::new)
    }
}
impl FromStr for v2::PullRequestBranchName {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(s).map_err(Error::new)
    }
}
impl FromStr for v2::Separator {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(s).map_err(Error::new)
    }
}
impl FromStr for v2::RebaseStrategy {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(s).map_err(Error::new)
    }
}
impl FromStr for v2::VersioningStrategy {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(s).map_err(Error::new)
    }
}
impl FromStr for v2::Registry {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(s).map_err(Error::new)
    }
}
impl FromStr for v2::RegistryType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(s).map_err(Error::new)
    }
}
