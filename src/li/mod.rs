use static_web_minify::minify_js_file;

mod broinfomaster;
pub use broinfomaster::*;

/// Returns `javascript` to get the `user agent`.
pub fn user_agent_js() -> &'static str {
    minify_js_file!("assets/js/user_agent.js")
}

/// Returns `javascript` to get the `browser info`.
pub fn broinfo_js() -> &'static str {
    minify_js_file!("assets/js/broinfo.js")
}
