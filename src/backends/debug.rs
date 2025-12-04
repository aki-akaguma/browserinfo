use anyhow::Result;

#[allow(unused_imports)]
use dioxus::prelude::*;

#[allow(unused_imports)]
use crate::utils::{BroInfo, BroInfoMaster};

#[cfg_attr(not(feature = "desktop"), server)]
pub async fn save_user_agent(ua: String) -> Result<()> {
    dioxus_logger::tracing::info!("save_user_agent: {ua:?}");
    //
    Ok(())
}

#[cfg_attr(not(feature = "desktop"), server)]
pub async fn save_broinfo(broinfo_m: BroInfoMaster) -> Result<()> {
    //dioxus_logger::tracing::debug!("save_broinfo: {broinfo_m:?}");
    //
    Ok(())
}
