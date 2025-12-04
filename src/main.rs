use dioxus::prelude::*;
#[cfg(all(not(debug_assertions), feature = "desktop"))]
use dioxus_desktop::{Config, WindowBuilder};

mod li;

use li::BrowserInfoCm;
use li::{BroInfo, Browser};
//pub use modules::browserinfo;

fn main() {
    // you can set the ports and IP manually with env vars:
    // server launch:
    // IP="0.0.0.0" PORT=8080 ./server

    #[cfg(any(
        not(debug_assertions),
        not(feature = "desktop"),
        not(feature = "server")
    ))]
    {
        //let backend_url = "https://hot-dog.fly.dev";
        let backend_url = "http://hcc-desktop.local:8080";
        dioxus_fullstack::set_server_url(backend_url);
    }

    #[cfg(not(debug_assertions))]
    let level = dioxus_logger::tracing::Level::INFO;
    #[cfg(debug_assertions)]
    let level = dioxus_logger::tracing::Level::DEBUG;
    dioxus_logger::init(level).expect("failed to init logger");

    #[cfg(any(debug_assertions, not(feature = "desktop")))]
    dioxus::launch(App);

    #[cfg(all(not(debug_assertions), feature = "desktop"))]
    dioxus::LaunchBuilder::new()
        .with_cfg(
            Config::default().with_menu(None).with_window(
                WindowBuilder::new()
                    .with_maximized(false)
                    .with_title("Tap tap tap beat"),
            ),
        )
        .launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link { rel: "preconnect", href: "https://fonts.gstatic.com", crossorigin: "" }
        document::Link { rel: "stylesheet", href: "https://fonts.googleapis.com/css2?family=Bagel+Fat+One:wght@400&display=swap" }

        MyStyle {}
        BroInfoHome {}
    }
}

#[cfg(not(feature = "inline_style"))]
#[component]
fn MyStyle() -> Element {
    rsx! {}
}

#[cfg(feature = "inline_style")]
#[component]
fn MyStyle() -> Element {
    rsx! {}
}

#[component]
fn BroInfoHome() -> Element {
    let broinfo_sig = use_signal(BroInfo::default);
    let browser_sig = use_signal(Browser::default);
    let brg = browser_sig.read().clone();
    let bim = broinfo_sig.read().clone();
    let brg_s = format!("{:?}", brg);
    let bim_s = format!("{:?}", bim);
    rsx! {
        BrowserInfoCm {
            broinfo: broinfo_sig,
            browser: browser_sig,
        }
        div {
            "{brg_s}"
        }
        div {}
        div {
            "{bim_s}"
        }
    }
}
