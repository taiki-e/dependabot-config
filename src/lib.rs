// SPDX-License-Identifier: Apache-2.0 OR MIT

/*!
Structured access to the [Dependabot] configuration file.

## Examples

```rust
# fn dox() -> std::io::Result<()> {
use std::fs;

use dependabot_config::v2::Dependabot;

let s = fs::read_to_string(".github/dependabot.yml")?;
let dependabot: Dependabot = s.parse()?;
for update in dependabot.updates {
    println!("{}", update.package_ecosystem);
}
# Ok(())
# }
```

[dependabot]: https://docs.github.com/en/code-security/supply-chain-security/about-dependabot-version-updates
*/

#![doc(test(
    no_crate_inject,
    attr(
        deny(warnings, rust_2018_idioms, single_use_lifetimes),
        allow(dead_code, unused_variables)
    )
))]
#![forbid(unsafe_code)]
#![warn(
    // Lints that may help when writing public library.
    missing_debug_implementations,
    missing_docs,
    clippy::alloc_instead_of_core,
    clippy::exhaustive_enums,
    clippy::exhaustive_structs,
    clippy::impl_trait_in_params,
    // clippy::missing_inline_in_public_items,
    // clippy::std_instead_of_alloc,
    // clippy::std_instead_of_core,
)]

#[cfg(test)]
#[path = "gen/assert_impl.rs"]
mod assert_impl;
#[path = "gen/display.rs"]
mod display;
#[path = "gen/from_str.rs"]
mod from_str;

mod error;

pub mod v1;
pub mod v2;

use serde_derive::{Deserialize, Serialize};

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

#[allow(clippy::to_string_trait_impl)]
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
