mod broinfomaster;
pub use broinfomaster::*;

/// Returns `javascript` to get the `user agent`.
pub fn user_agent_js() -> &'static str {
    include_str!("../../assets/min/user_agent.js")
}

/// Returns `javascript` to get the `browser info`.
pub fn broinfo_js() -> &'static str {
    include_str!("../../assets/min/broinfo.js")
}
