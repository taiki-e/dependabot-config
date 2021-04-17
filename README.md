# dependabot-config

[![crates.io](https://img.shields.io/crates/v/dependabot-config?style=flat-square&logo=rust)](https://crates.io/crates/dependabot-config)
[![docs.rs](https://img.shields.io/badge/docs.rs-dependabot--config-blue?style=flat-square)](https://docs.rs/dependabot-config)
[![license](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue?style=flat-square)](#license)
[![rustc](https://img.shields.io/badge/rustc-1.51+-blue?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![build status](https://img.shields.io/github/workflow/status/taiki-e/dependabot-config/CI/main?style=flat-square&logo=github)](https://github.com/taiki-e/dependabot-config/actions)

Structured access to the [Dependabot] configuration file.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
dependabot-config = "0.2"
```

*Compiler support: requires rustc 1.51+*

## Examples

```rust
use std::fs;

use dependabot_config::v2::Dependabot;

let s = fs::read_to_string(".github/dependabot.yml")?;
let dependabot: Dependabot = s.parse()?;
for update in dependabot.updates {
    println!("{}", update.package_ecosystem);
}
```

[dependabot]: https://docs.github.com/en/code-security/supply-chain-security/about-dependabot-version-updates

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
