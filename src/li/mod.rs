use anyhow::Result;
use dioxus::prelude::*;
use static_web_minify::minify_js_file;

mod broinfomaster;
pub use broinfomaster::*;

mod backends;

#[derive(Props, Debug, Clone, PartialEq)]
pub struct BrowserInfoProps {
    broinfo: Signal<BroInfo>,
    browser: Signal<Browser>,
}

#[component]
pub fn BrowserInfoCm(mut props: BrowserInfoProps) -> Element {
    use_future(move || async move {
        let (broinfo, browser) = browserinfo().await.unwrap();
        props.broinfo.set(broinfo);
        props.browser.set(browser);
    });

    rsx! {}
}

pub async fn browserinfo() -> Result<(BroInfo, Browser)> {
    /*
    let js_ua: &str = minify_js_file!("assets/js/user_agent.js");
    let v = document::eval(js_ua).await?;
    let s = v.to_string();
    let user_agent: UserAgent = serde_json::from_str(&s)?;
    let _ = backends::save_user_agent(user_agent).await;
    */
    //
    let js_bro: &str = minify_js_file!("assets/js/broinfo.js");
    let v = document::eval(js_bro).await?;
    let s = v.to_string();
    let broinfo: BroInfo = serde_json::from_str(&s)?;
    //dioxus_logger::tracing::debug!("{s:?}");
    let browser = backends::save_broinfo(broinfo.clone(), true)
        .await?
        .unwrap();
    Ok((broinfo, browser))
}
