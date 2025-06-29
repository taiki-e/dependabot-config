# dependabot-config

[![crates.io](https://img.shields.io/crates/v/dependabot-config?style=flat-square&logo=rust)](https://crates.io/crates/dependabot-config)
[![docs.rs](https://img.shields.io/badge/docs.rs-dependabot--config-blue?style=flat-square&logo=docs.rs)](https://docs.rs/dependabot-config)
[![license](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue?style=flat-square)](#license)
[![msrv](https://img.shields.io/badge/msrv-1.65-blue?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![github actions](https://img.shields.io/github/actions/workflow/status/taiki-e/dependabot-config/ci.yml?branch=main&style=flat-square&logo=github)](https://github.com/taiki-e/dependabot-config/actions)

<!-- tidy:sync-markdown-to-rustdoc:start:src/lib.rs -->

Structured access to the [Dependabot] configuration file.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
dependabot-config = "0.3"
```

## Examples

```rust
use std::fs;

use dependabot_config::v2::Dependabot;

let s = fs::read_to_string(".github/dependabot.yml").unwrap();
let dependabot: Dependabot = s.parse().unwrap();
for update in dependabot.updates {
    println!("{}", update.package_ecosystem);
}
```

[dependabot]: https://docs.github.com/en/code-security/dependabot/dependabot-version-updates/about-dependabot-version-updates

<!-- tidy:sync-markdown-to-rustdoc:end -->

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
