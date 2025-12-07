/*!
Provides the `javascript` string and `rust` `struct` that are required when running `javascript` and collecting information.

# How to use
In `dioxus`, it is used as follows.

### user agent
```rust
use dioxus_document as document;
use browserinfo::{user_agent_js, UserAgent};

# async fn func() -> anyhow::Result<()> {
let js_ua: &str = user_agent_js();
let eval = document::eval(js_ua).await?;
let json_str = eval.to_string();
let user_agent: UserAgent = serde_json::from_str(&json_str)?;
# Ok(())
# }
```

### browser info
```rust
use dioxus_document as document;
use browserinfo::{broinfo_js, BroInfo, Browser};

# async fn func() -> anyhow::Result<()> {
let js_bro: &str = broinfo_js();
let eval = document::eval(js_bro).await?;
let json_str = eval.to_string();
let broinfo: BroInfo = serde_json::from_str(&json_str)?;

// Generate `Browser` from `UserAgent`
let browser = broinfo.to_browser();
# Ok(())
# }
```
*/

mod li;
pub use li::*;
