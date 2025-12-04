//use anyhow::Result;
use dioxus::prelude::*;
//use static_web_minify::minify_js_file;

mod broinfomaster;
pub use broinfomaster::*;

mod backends;
//pub use backends::*;

#[derive(Props, Clone, PartialEq)]
pub struct InfoProps {
    broinfo: Signal<BroInfo>,
    browser: Signal<Browser>,
}

#[component]
pub fn BrowserInfo(mut props: InfoProps) -> Element {
    use_future(move || async move {
        //let js_ua: &str = minify_js_file!("assets/js/user_agent.js");
        let js_ua: &str = include_str!("../assets/js/user_agent.js");
        //const _S1: &str = include_str!("../assets/js/user_agent.js");
        //let js_ua: &str = minify_js_str!(_S1);
        let v = document::eval(js_ua).await.unwrap();
        let s = v.to_string();
        let user_agent: UserAgent = serde_json::from_str(&s).unwrap();
        _ = backends::save_user_agent(user_agent).await;
        //
        //let js_bro: &str = minify_js_file!("assets/js/broinfo.js");
        let js_bro: &str = include_str!("../assets/js/broinfo.js");
        //const _S2: &str = include_str!("../assets/js/broinfo.js");
        //let js_bro: &str = minify_js_str!(_S2);
        let v = document::eval(js_bro).await.unwrap();
        let s = v.to_string();
        let broinfo: BroInfo = serde_json::from_str(&s).unwrap();
        //dioxus_logger::tracing::debug!("{s:?}");
        let browser = backends::save_broinfo(broinfo.clone(), true)
            .await
            .unwrap()
            .unwrap();
        //
        props.broinfo.set(broinfo);
        props.browser.set(browser);
    });

    rsx! {}
}

/*
#[cfg(not(feature = "server"))]
pub async fn browserinfo() -> Result<(BroInfo, Browser)> {
    //let js_ua: &str = minify_js_file!("packages/browserinfo/assets/js/user_agent.js");
    const _S1: &str = include_str!("../assets/js/user_agent.js");
    let js_ua: &str = minify_js_str!(_S1);
    let v = document::eval(js_ua).await?;
    let s = v.to_string();
    let user_agent: UserAgent = serde_json::from_str(&s)?;
    _ = crate::backends::save_user_agent(user_agent).await;
    //
    const _S2: &str = include_str!("../assets/js/broinfo.js");
    let js_bro: &str = minify_js_str!(_S2);
    let v = document::eval(js_bro).await?;
    let s = v.to_string();
    let broinfo: BroInfo = serde_json::from_str(&s)?;
    //dioxus_logger::tracing::debug!("{s:?}");
    let browser = crate::backends::save_broinfo(broinfo.clone(), true)
        .await?
        .unwrap();
    Ok((broinfo, browser))
}
*/
