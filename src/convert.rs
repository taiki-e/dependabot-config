use std::convert::TryFrom;

use crate::{v1, v2, Error};

impl TryFrom<v2::VersioningStrategy> for v1::VersionRequirementUpdate {
    type Error = Error;
    fn try_from(v2: v2::VersioningStrategy) -> Result<Self, Self::Error> {
        Ok(match v2 {
            v2::VersioningStrategy::LockfileOnly => v1::VersionRequirementUpdate::Off,
            v2::VersioningStrategy::Auto => v1::VersionRequirementUpdate::Auto,
            v2::VersioningStrategy::Widen => v1::VersionRequirementUpdate::WidenRanges,
            v2::VersioningStrategy::Increase => v1::VersionRequirementUpdate::IncreaseVersions,
            v2::VersioningStrategy::IncreaseIfNecessary => {
                v1::VersionRequirementUpdate::IncreaseVersionsIfNecessary
            }
        })
    }
}
impl TryFrom<v1::VersionRequirementUpdate> for v2::VersioningStrategy {
    type Error = Error;
    fn try_from(v1: v1::VersionRequirementUpdate) -> Result<Self, Self::Error> {
        Ok(match v1 {
            v1::VersionRequirementUpdate::Off => v2::VersioningStrategy::LockfileOnly,
            v1::VersionRequirementUpdate::Auto => v2::VersioningStrategy::Auto,
            v1::VersionRequirementUpdate::WidenRanges => v2::VersioningStrategy::Widen,
            v1::VersionRequirementUpdate::IncreaseVersions => v2::VersioningStrategy::Increase,
            v1::VersionRequirementUpdate::IncreaseVersionsIfNecessary => {
                v2::VersioningStrategy::IncreaseIfNecessary
            }
        })
    }
}

impl TryFrom<v2::CommitMessage> for v1::CommitMessage {
    type Error = Error;
    fn try_from(v2: v2::CommitMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            prefix: v2.prefix,
            prefix_development: v2.prefix_development,
            include_scope: v2.include.map(|include| include == v2::CommitMessageInclude::Scope),
        })
    }
}
impl TryFrom<v1::CommitMessage> for v2::CommitMessage {
    type Error = Error;
    fn try_from(v1: v1::CommitMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            prefix: v1.prefix,
            prefix_development: v1.prefix_development,
            include: v1.include_scope.and_then(|include| {
                if include { Some(v2::CommitMessageInclude::Scope) } else { None }
            }),
        })
    }
}
