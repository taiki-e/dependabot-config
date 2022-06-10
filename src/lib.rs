//! Structured access to the Dependabot configuration.
//!
//! ## Examples
//!
//! ```rust
//! # fn dox() -> std::io::Result<()> {
//! use std::fs;
//!
//! use dependabot_config::v2::Dependabot;
//!
//! let s = fs::read_to_string(".github/dependabot.yml")?;
//! let dependabot: Dependabot = s.parse()?;
//! for update in dependabot.updates {
//!     println!("{}", update.package_ecosystem);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! [dependabot]: https://docs.github.com/en/code-security/supply-chain-security/about-dependabot-version-updates

#![doc(test(
    no_crate_inject,
    attr(
        deny(warnings, rust_2018_idioms, single_use_lifetimes),
        allow(dead_code, unused_variables)
    )
))]
#![forbid(unsafe_code)]
#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    single_use_lifetimes,
    unreachable_pub
)]
#![warn(clippy::default_trait_access, clippy::wildcard_imports)]

#[cfg(test)]
#[rustfmt::skip]
#[path = "gen/assert_impl.rs"]
mod assert_impl;
#[rustfmt::skip]
#[path = "gen/display.rs"]
mod display;
#[rustfmt::skip]
#[path = "gen/from_str.rs"]
mod from_str;

// TODO: https://github.com/taiki-e/dependabot-config/issues/2
// mod convert;
mod error;

pub mod v1;
pub mod v2;

use serde::{Deserialize, Serialize};

pub use crate::error::Error;

/// The Dependabot configuration.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum Dependabot {
    /// The Dependabot v2 configuration.
    V2(v2::Dependabot),
    /// The Dependabot v1 configuration.
    V1(v1::Dependabot),
}

impl ToString for Dependabot {
    fn to_string(&self) -> String {
        serde_yaml::to_string(&self).unwrap()
    }
}

impl From<v1::Dependabot> for Dependabot {
    fn from(v1: v1::Dependabot) -> Self {
        Self::V1(v1)
    }
}

impl From<v2::Dependabot> for Dependabot {
    fn from(v2: v2::Dependabot) -> Self {
        Self::V2(v2)
    }
}
