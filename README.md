# browserinfo

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![Rust Version][rustc-image]
![Apache2/MIT licensed][license-image]
[![Test ubu][test-ubuntu-image]][test-ubuntu-link]
[![Test mac][test-windows-image]][test-windows-link]
[![Test win][test-macos-image]][test-macos-link]

Provides the `javascript` string and `rust` `struct` that are required when running `javascript` and collecting information.

## How to use
In `dioxus`, it is used as follows.

#### user agent
```rust
use dioxus_document as document;
use browserinfo::{user_agent_js, UserAgent};

let js_ua: &str = user_agent_js();
let eval = document::eval(js_ua).await?;
let json_str = eval.to_string();
let user_agent = UserAgent::from_json_str(&json_str)?;
```

#### browser info
```rust
use dioxus_document as document;
use browserinfo::{broinfo_js, BroInfo, Browser};

let js_bro: &str = broinfo_js();
let eval = document::eval(js_bro).await?;
let json_str = eval.to_string();
let broinfo = BroInfo::from_json_str(&json_str)?;

// Generate `Browser` from `UserAgent`
let browser = broinfo.to_browser();
```

# Changelogs

[This crate's changelog here.](https://github.com/aki-akaguma/browserinfo/blob/main/CHANGELOG.md)

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)

at your option.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/browserinfo.svg
[crate-link]: https://crates.io/crates/browserinfo
[docs-image]: https://docs.rs/browserinfo/badge.svg
[docs-link]: https://docs.rs/browserinfo/
[rustc-image]: https://img.shields.io/badge/rustc-1.90+-blue.svg
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[test-ubuntu-image]: https://github.com/aki-akaguma/browserinfo/actions/workflows/test-ubuntu.yml/badge.svg
[test-ubuntu-link]: https://github.com/aki-akaguma/browserinfo/actions/workflows/test-ubuntu.yml
[test-macos-image]: https://github.com/aki-akaguma/browserinfo/actions/workflows/test-macos.yml/badge.svg
[test-macos-link]: https://github.com/aki-akaguma/browserinfo/actions/workflows/test-macos.yml
[test-windows-image]: https://github.com/aki-akaguma/browserinfo/actions/workflows/test-windows.yml/badge.svg
[test-windows-link]: https://github.com/aki-akaguma/browserinfo/actions/workflows/test-windows.yml
