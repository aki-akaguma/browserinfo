use anyhow::Result;
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use dioxus::prelude::*;

#[allow(unused_imports)]
use crate::utils::{BroInfo, BroInfoMaster};

const NEXT_URL: &str = "http://hcc-desktop.local:8080";
//const NEXT_URL: &str = "http://192.168.116.102:8080";
const NEXT_URL_API_TAG: &str = "2751944067970052790";

#[cfg_attr(not(feature = "desktop"), server)]
pub async fn save_user_agent(ua: String) -> Result<()> {
    use std::time::Duration;

    #[derive(Serialize, Deserialize, Debug, Default, Clone)]
    struct UserAgentProps {
        pub ua: String,
    }
    let a_props = UserAgentProps { ua };

    let client = reqwest::Client::builder()
        .connect_timeout(Duration::from_millis(1000))
        .build()?;
    let _res = client
        .post(&format!("{NEXT_URL}/api/save_user_agent{NEXT_URL_API_TAG}"))
        .header("x-request-client", "dioxus")
        .timeout(Duration::from_millis(5000))
        .json(&a_props)
        .send()
        .await?;
    //dioxus_logger::tracing::info!("save_user_agent next: {_res:?}");
    Ok(())
}

#[cfg_attr(not(feature = "desktop"), server)]
pub async fn save_broinfo(broinfo_m: BroInfoMaster) -> Result<()> {
    use std::time::Duration;

    #[derive(Serialize, Deserialize, Debug, Default, Clone)]
    struct BroInfoProps {
        pub broinfo_m: BroInfoMaster,
    }
    let a_props = BroInfoProps { broinfo_m };

    let client = reqwest::Client::builder()
        .connect_timeout(Duration::from_millis(1000))
        .build()?;

    let _res = client
        .post(&format!("{NEXT_URL}/api/save_broinfo{NEXT_URL_API_TAG}"))
        .header("x-request-client", "dioxus")
        .timeout(Duration::from_millis(5000))
        .json(&a_props)
        .send()
        .await;
    //dioxus_logger::tracing::info!("save_broinfo next: {_res:?}");
    Ok(())
}
