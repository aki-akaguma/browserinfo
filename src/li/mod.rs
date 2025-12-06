use static_web_minify::minify_js_file;

mod broinfomaster;
pub use broinfomaster::*;

pub fn user_agent_js() -> &'static str {
    minify_js_file!("assets/js/user_agent.js")
}

pub fn broinfo_js() -> &'static str {
    minify_js_file!("assets/js/broinfo.js")
}
