use anyhow::Result;
use serde::{Deserialize, Serialize};

/// This is information obtained with `javascript`
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BroInfo {
    pub basic: Basic,
    pub jsinfo: JsInfo,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Basic {
    pub user_agent: UserAgent,
    pub referrer: Referrer,
}

//
macro_rules! SingleTypeString {
    ($ty: ident) => {
        /// new type idiom: a single type string
        #[derive(Serialize, Deserialize, Debug, Default, Clone)]
        pub struct $ty(String);
        impl $ty {
            /// Creates an object containing one `String`.
            pub fn new(val: String) -> Self {
                Self(val)
            }
            //#[allow(dead_code)]
            /// Returns true if self has a length of zero bytes.
            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }
            //#[allow(dead_code)]
            /// Returns a reference (`&str`) of the contained `String`.
            pub fn get(&self) -> &str {
                self.0.as_str()
            }
        }

        impl std::fmt::Display for $ty {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl std::convert::From<&str> for $ty {
            fn from(val: &str) -> Self {
                $ty::new(val.to_string())
            }
        }
    };
}

SingleTypeString!(UserAgent);
SingleTypeString!(Referrer);

/// This is information obtained with `javascript`
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct JsInfo {
    pub oscpu: String,
    pub platform: String,
    pub cpu_cores: Option<i32>,
    pub cookie_enabled: bool,
    pub user_language: String,
    pub device_memory: Option<i32>,
    pub screen_width: Option<i32>,
    pub screen_height: Option<i32>,
    pub screen_color_depth: Option<i32>,
    pub device_pixcel_ratio: Option<f64>,
    pub has_local_storage: bool,
    pub has_session_storage: bool,
    pub timezone: String,
}

/// The browser information.
/// This is the information obtained by parsing `user agent`
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Browser {
    /// a browser name
    pub name: String,
    /// a browser version
    pub version: String,
    /// a operating system
    pub os: Option<Os>,
    /// a device model
    pub device: String,
}

/// The operating system information.
/// This is the information obtained by parsing `user agent`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Os {
    /// a operating system name
    pub name: String,
    /// a operating system version
    pub version: String,
}

impl BroInfo {
    /// Parses `user agent` and generates `Browser`.
    //#[allow(dead_code)]
    pub fn to_browser(&self) -> Result<Browser> {
        convert_from_user_agent(self.basic.user_agent.get())
    }
}

//
// To get the latest `regexes.yaml` from the `ua` parser community:
//   git submodule update --init
//
static EXTRACTOR: std::sync::LazyLock<ua_parser::Extractor> = std::sync::LazyLock::new(|| {
    let s = include_str!("../../resources/regexes.yaml");
    let regexes: ua_parser::Regexes = serde_yaml::from_str(s).unwrap();
    ua_parser::Extractor::try_from(regexes).unwrap()
});

#[allow(dead_code)]
fn get_extractor<'a>() -> Result<&'a ua_parser::Extractor<'a>> {
    Ok(&*EXTRACTOR)
}

#[allow(dead_code)]
fn convert_from_user_agent(ua: &str) -> Result<Browser> {
    let extractor = get_extractor()?;
    let (browser, os, device) = extractor.extract(ua);

    let (name, version) = if let Some(browser) = browser {
        let name = format!("{}", browser.family);
        let version = if let Some(major) = browser.major {
            if let Some(minor) = browser.minor {
                if let Some(patch) = browser.patch {
                    if let Some(patch_minor) = browser.patch_minor {
                        format!("{major}.{minor}.{patch}.{patch_minor}")
                    } else {
                        format!("{major}.{minor}.{patch}")
                    }
                } else {
                    format!("{major}.{minor}")
                }
            } else {
                major.to_string()
            }
        } else {
            String::new()
        };
        (name, version)
    } else {
        (String::new(), String::new())
    };

    let (os_name, os_version) = if let Some(os) = os {
        let os_name = format!("{}", os.os);
        let os_version = if let Some(major) = os.major {
            if let Some(minor) = os.minor {
                if let Some(patch) = os.patch {
                    if let Some(patch_minor) = os.patch_minor {
                        format!("{major}.{minor}.{patch}.{patch_minor}")
                    } else {
                        format!("{major}.{minor}.{patch}")
                    }
                } else {
                    format!("{major}.{minor}")
                }
            } else {
                format!("{major}")
            }
        } else {
            String::new()
        };
        (os_name, os_version)
    } else {
        (String::new(), String::new())
    };

    let device_model = if let Some(dev) = device {
        let brand_model = if let Some(brand) = dev.brand {
            if let Some(model) = dev.model {
                if let Some(idx) = model.rfind(r" Build/") {
                    format!("{brand}/{}", &model[..idx])
                } else {
                    format!("{brand}/{model}")
                }
            } else {
                format!("{brand}")
            }
        } else if let Some(model) = dev.model {
            if let Some(idx) = model.rfind(r" Build/") {
                model[..idx].to_string()
            } else {
                model.to_string()
            }
        } else {
            String::new()
        };
        let devdev = if let Some(idx) = dev.device.rfind(r" Build/") {
            dev.device[..idx].to_string()
        } else {
            dev.device.to_string()
        };
        if brand_model.len() >= devdev.len() {
            brand_model
        } else {
            devdev
        }
    } else {
        String::new()
    };

    //
    Ok(Browser {
        name,
        version,
        os: Some(Os {
            name: os_name,
            version: os_version,
        }),
        device: device_model,
    })
}

// for test
#[allow(unused_macros)]
macro_rules! break_dump_extractor {
    ($ua:expr) => {{
        let extractor = get_extractor().unwrap();
        let (browser, os, device) = extractor.extract($ua);
        assert_eq!(format!("{browser:?},{os:?},{device:?}"), "");
    }};
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_01() {
        let broinfo = BroInfo::default();
        let s = serde_json::to_string(&broinfo).unwrap();
        assert_eq!(
            s,
            r#"{"basic":{"user_agent":"","referrer":""},"jsinfo":{"oscpu":"","platform":"","cpu_cores":null,"cookie_enabled":false,"user_language":"","device_memory":null,"screen_width":null,"screen_height":null,"screen_color_depth":null,"device_pixcel_ratio":null,"has_local_storage":false,"has_session_storage":false,"timezone":""}}"#
        );
    }
    #[test]
    fn test_02() {
        let s0 = r#"{"basic":{"user_agent":"Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:145.0) Gecko/20100101 Firefox/145.0","referrer":"http://test.test/xxxx.html"},"jsinfo":{"oscpu":"intel","platform":"Linux x86_64","cpu_cores":4,"cookie_enabled":true,"user_language":"ja_JP","device_memory":8,"screen_width":1480,"screen_height":960,"screen_color_depth":8,"device_pixcel_ratio":1.0,"has_local_storage":true,"has_session_storage":true,"timezone":"Asia/Tokyo"}}"#;
        let broinfo: BroInfo = serde_json::from_str(s0).unwrap();
        let s = serde_json::to_string(&broinfo).unwrap();
        assert_eq!(s, s0);
    }
    #[test]
    fn test_user_agent_00() {
        let s0 = r#""#;
        let browser = convert_from_user_agent(s0.into()).unwrap();
        let target = concat!(
            r#"Browser { name: "", version: "", "#,
            r#"os: Some(Os { name: "", version: "" }), "#,
            r#"device: "" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);
    }
    #[test]
    fn test_user_agent_on_linux_01() {
        // linux desktop browser: firefox
        let s0 =
            r#"Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:145.0) Gecko/20100101 Firefox/145.0"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Firefox", version: "145.0", "#,
            r#"os: Some(Os { name: "Ubuntu", version: "" }), "#,
            r#"device: "" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);
    }
    #[test]
    fn test_user_agent_on_linux_02() {
        // linux desktop browser: chrome
        let s0 = r#"Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/142.0.0.0 Safari/537.36"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome", version: "142.0.0.0", "#,
            r#"os: Some(Os { name: "Linux", version: "" }), "#,
            r#"device: "" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);
    }
    #[test]
    fn test_user_agent_on_linux_03() {
        // linux desktop appli
        let s0 = r#"Mozilla/5.0 (X11; Ubuntu; Linux x86_64) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/60.5 Safari/605.1.15"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Safari", version: "60.5", "#,
            r#"os: Some(Os { name: "Ubuntu", version: "" }), "#,
            r#"device: "" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);
    }
    #[test]
    fn test_user_agent_on_windows_01() {
        // windows desktop browser: firefox
        let s0 =
            r#"Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:145.0) Gecko/20100101 Firefox/145.0"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Firefox", version: "145.0", "#,
            r#"os: Some(Os { name: "Windows", version: "10" }), "#,
            r#"device: "" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);
    }
    #[test]
    fn test_user_agent_on_windows_02() {
        // windows desktop browser: edge
        let s0 = r#"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/142.0.0.0 Safari/537.36 Edg/142.0.0.0"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Edge", version: "142.0.0.0", "#,
            r#"os: Some(Os { name: "Windows", version: "10" }), "#,
            r#"device: "" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);
    }
    #[test]
    fn test_user_agent_on_android_01() {
        // android browser: chrome
        let s0 = r#"Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/142.0.0.0 Mobile Safari/537.36"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile", version: "142.0.0.0", "#,
            r#"os: Some(Os { name: "Android", version: "10" }), "#,
            r#"device: "Generic_Android/K" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);
    }
    #[test]
    fn test_user_agent_on_android_02() {
        // android aplli
        let s0 = r#"Mozilla/5.0 (Linux; Android 11; S5-SH Build/S2014; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/142.0.7444.102 Mobile Safari/537.36"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile WebView", version: "142.0.7444.102", "#,
            r#"os: Some(Os { name: "Android", version: "11" }), "#,
            r#"device: "Generic_Android/S5-SH" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);
        //break_dump_extractor!(s0);
    }
    #[test]
    fn test_user_agent_on_android_emu4_01() {
        // android-emu Pixcel_4 browser: chrome
        let s0 = r#"Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/142.0.0.0 Mobile Safari/537.36"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile", version: "142.0.0.0", "#,
            r#"os: Some(Os { name: "Android", version: "10" }), "#,
            r#"device: "Generic_Android/K" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);
    }
    #[test]
    fn test_user_agent_on_android_emu4_02() {
        // android-emu Pixcel_4 appli
        let s0 = r#"Mozilla/5.0 (Linux; Android 11; sdk_gphone_x86_64 Build/RSR1.201211.001; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/141.0.7390.122 Mobile Safari/537.36"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile WebView", version: "141.0.7390.122", "#,
            r#"os: Some(Os { name: "Android", version: "11" }), "#,
            r#"device: "Generic_Android/sdk_gphone_x86_64" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);
    }
    #[test]
    fn test_user_agent_on_android_emu7_01() {
        // android-emu Pixcel_7 browser: chrome
        let s0 = r#"Mozilla/5.0 (Linux; Android 13; sdk_gphone64_x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Mobile Safari/537.36"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile", version: "109.0.0.0", "#,
            r#"os: Some(Os { name: "Android", version: "13" }), "#,
            r#"device: "Generic_Android/sdk_gphone64_x86_64" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);
    }
    #[test]
    fn test_user_agent_on_android_emu7_02() {
        // android-emu Pixcel_7 appli
        let s0 = r#"Mozilla/5.0 (Linux; Android 13; sdk_gphone64_x86_64 Build/TE1A.240213.009; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/142.0.7444.102 Mobile Safari/537.36"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile WebView", version: "142.0.7444.102", "#,
            r#"os: Some(Os { name: "Android", version: "13" }), "#,
            r#"device: "Generic_Android/sdk_gphone64_x86_64" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);
    }
}

#[cfg(test)]
mod test_more_01 {
    use super::*;
    #[test]
    fn test_user_agent_on_android_samsung_01() {
        // android: Samsung Galaxy S25
        let s0 = r#"Mozilla/5.0 (Linux; Android 15; SM-S931B Build/AP3A.240905.015.A2; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/127.0.6533.103 Mobile Safari/537.36"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile WebView", version: "127.0.6533.103", "#,
            r#"os: Some(Os { name: "Android", version: "15" }), "#,
            r#"device: "Samsung/SM-S931B" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);

        let s0 = r#"Mozilla/5.0 (Linux; Android 15; SM-S931U Build/AP3A.240905.015.A2; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/132.0.6834.163 Mobile Safari/537.36"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile WebView", version: "132.0.6834.163", "#,
            r#"os: Some(Os { name: "Android", version: "15" }), "#,
            r#"device: "Samsung/SM-S931U" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);
    }
    #[test]
    fn test_user_agent_on_android_samsung_02() {
        // android: Samsung Galaxy S24 Ultra
        let s0 = r#"Mozila/5.0 (Linux; Android 14; SM-S928B/DS) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.6099.230 Mobile Safari/537.36"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile", version: "120.0.6099.230", "#,
            r#"os: Some(Os { name: "Android", version: "14" }), "#,
            r#"device: "Generic_Android/SM-S928B/DS" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);
        //break_dump_extractor!(s0);

        let s0 = r#"Mozila/5.0 (Linux; Android 14; SM-S928W) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.6099.230 Mobile Safari/537.36"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile", version: "120.0.6099.230", "#,
            r#"os: Some(Os { name: "Android", version: "14" }), "#,
            r#"device: "Samsung/SM-S928W" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);
    }
    #[test]
    fn test_user_agent_on_android_samsung_03() {
        // android: Samsung Flip
        let s0 = r#"Mozilla/5.0 (Linux; Android 14; SM-F9560 Build/UP1A.231005.007; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/127.0.6533.103 Mobile Safari/537.36"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile WebView", version: "127.0.6533.103", "#,
            r#"os: Some(Os { name: "Android", version: "14" }), "#,
            r#"device: "Samsung/SM-F9560" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);

        let s0 = r#"Mozilla/5.0 (Linux; Android 14; SM-F956U) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/80.0.3987.119 Mobile Safari/537.36"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile WebView", version: "80.0.3987.119", "#,
            r#"os: Some(Os { name: "Android", version: "14" }), "#,
            r#"device: "Samsung/SM-F956U" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);
    }
    #[test]
    fn test_user_agent_on_android_samsung_04() {
        // android: Samsung Galaxy Xcover7
        let s0 =
            r#"Mozilla/5.0 (Android 15; Mobile; SM-G556B/DS; rv:130.0) Gecko/130.0 Firefox/130.0"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Firefox Mobile", version: "130.0", "#,
            r#"os: Some(Os { name: "Android", version: "15" }), "#,
            r#"device: "Generic/Smartphone" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);

        let s0 =
            r#"Mozilla/5.0 (Android 15; Mobile; SM-G556B; rv:130.0) Gecko/130.0 Firefox/130.0"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Firefox Mobile", version: "130.0", "#,
            r#"os: Some(Os { name: "Android", version: "15" }), "#,
            r#"device: "Generic/Smartphone" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);
    }
    #[test]
    fn test_user_agent_on_android_samsung_05() {
        // android: Samsung Galaxy S23
        let s0 = r#"Mozilla/5.0 (Linux; Android 13; SM-S911B) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/104.0.0.0 Mobile Safari/537.36 Dalvik/2.1.0 (Linux; U; Android 13; SM-S911B Build/TP1A.220624.014)"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile", version: "104.0.0.0", "#,
            r#"os: Some(Os { name: "Android", version: "13" }), "#,
            r#"device: "Samsung/SM-S911B" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);

        let s0 = r#"Mozilla/5.0 (Linux; Android 13; SM-S911U) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Mobile Safari/537.36"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile", version: "110.0.0.0", "#,
            r#"os: Some(Os { name: "Android", version: "13" }), "#,
            r#"device: "Samsung/SM-S911U" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);
    }
    #[test]
    fn test_user_agent_on_android_samsung_06() {
        // android: Samsung Galaxy S22 5G
        let s0 = r#"
        Mozilla/5.0 (Linux; Android 13; SM-S901B) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Mobile Safari/537.36
            "#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile", version: "112.0.0.0", "#,
            r#"os: Some(Os { name: "Android", version: "13" }), "#,
            r#"device: "Samsung/SM-S901B" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);

        let s0 = r#"
        Mozilla/5.0 (Linux; Android 13; SM-S901U) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Mobile Safari/537.36
            "#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile", version: "112.0.0.0", "#,
            r#"os: Some(Os { name: "Android", version: "13" }), "#,
            r#"device: "Samsung/SM-S901U" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);
    }
    #[test]
    fn test_user_agent_on_android_samsung_07() {
        // android: Samsung Galaxy S22 Ultra 5G
        let s0 = r#"Mozilla/5.0 (Linux; Android 13; SM-S908B) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Mobile Safari/537.36"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile", version: "112.0.0.0", "#,
            r#"os: Some(Os { name: "Android", version: "13" }), "#,
            r#"device: "Samsung/SM-S908B" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);

        let s0 = r#"Mozilla/5.0 (Linux; Android 13; SM-S908U) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/111.0.0.0 Mobile Safari/537.36"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile", version: "111.0.0.0", "#,
            r#"os: Some(Os { name: "Android", version: "13" }), "#,
            r#"device: "Samsung/SM-S908U" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);
    }
    #[test]
    fn test_user_agent_on_android_samsung_08() {
        // android: Samsung Galaxy S21 5G
        let s0 = r#"Mozilla/5.0 (Linux; Android 13; SM-G991B) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Mobile Safari/537.36"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile", version: "112.0.0.0", "#,
            r#"os: Some(Os { name: "Android", version: "13" }), "#,
            r#"device: "Samsung/SM-G991B" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);

        let s0 = r#"Mozilla/5.0 (Linux; Android 13; SM-G991U) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Mobile Safari/537.36"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile", version: "112.0.0.0", "#,
            r#"os: Some(Os { name: "Android", version: "13" }), "#,
            r#"device: "Samsung/SM-G991U" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);
    }
    #[test]
    fn test_user_agent_on_android_samsung_09() {
        // android: Samsung Galaxy S21 Ultra 5G
        let s0 = r#"Mozilla/5.0 (Linux; Android 13; SM-G998B) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Mobile Safari/537.36"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile", version: "112.0.0.0", "#,
            r#"os: Some(Os { name: "Android", version: "13" }), "#,
            r#"device: "Samsung/SM-G998B" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);

        let s0 = r#"Mozilla/5.0 (Linux; Android 13; SM-G998U) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Mobile Safari/537.36"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile", version: "112.0.0.0", "#,
            r#"os: Some(Os { name: "Android", version: "13" }), "#,
            r#"device: "Samsung/SM-G998U" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);
    }
    #[test]
    fn test_user_agent_on_android_samsung_10() {
        // android: Samsung Galaxy A53 5G
        let s0 = r#"
        Mozilla/5.0 (Linux; Android 13; SM-A536B) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Mobile Safari/537.36
            "#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile", version: "112.0.0.0", "#,
            r#"os: Some(Os { name: "Android", version: "13" }), "#,
            r#"device: "Samsung/SM-A536B" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);

        let s0 = r#"
        Mozilla/5.0 (Linux; Android 13; SM-A536U) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Mobile Safari/537.36
            "#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile", version: "112.0.0.0", "#,
            r#"os: Some(Os { name: "Android", version: "13" }), "#,
            r#"device: "Samsung/SM-A536U" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);
    }
    #[test]
    fn test_user_agent_on_android_samsung_11() {
        // android: Samsung Galaxy A51
        let s0 = r#"
        Mozilla/5.0 (Linux; Android 13; SM-A515F) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Mobile Safari/537.36
            "#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile", version: "112.0.0.0", "#,
            r#"os: Some(Os { name: "Android", version: "13" }), "#,
            r#"device: "Samsung/SM-A515F" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);

        let s0 = r#"
        Mozilla/5.0 (Linux; Android 13; SM-A515U) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Mobile Safari/537.36
            "#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile", version: "112.0.0.0", "#,
            r#"os: Some(Os { name: "Android", version: "13" }), "#,
            r#"device: "Samsung/SM-A515U" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);
    }
    #[test]
    fn test_user_agent_on_android_samsung_12() {
        // android: Samsung Galaxy S10
        let s0 = r#"
        Mozilla/5.0 (Linux; Android 12; SM-G973F) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Mobile Safari/537.36
            "#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile", version: "112.0.0.0", "#,
            r#"os: Some(Os { name: "Android", version: "12" }), "#,
            r#"device: "Samsung/SM-G973F" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);

        let s0 = r#"Mozilla/5.0 (Linux; Android 12; SM-G973U) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Mobile Safari/537.36"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile", version: "112.0.0.0", "#,
            r#"os: Some(Os { name: "Android", version: "12" }), "#,
            r#"device: "Samsung/SM-G973U" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);
    }
}

#[cfg(test)]
mod test_more_02 {
    use super::*;
    #[test]
    fn test_user_agent_on_android_google_01() {
        // android: Google Pixel 9 Pro
        let s0 = r#"Mozilla/5.0 (Linux; Android 14; Pixel 9 Pro Build/AD1A.240418.003; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/124.0.6367.54 Mobile Safari/537.36"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile WebView", version: "124.0.6367.54", "#,
            r#"os: Some(Os { name: "Android", version: "14" }), "#,
            r#"device: "Google/Pixel 9 Pro" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);

        // android: Google Pixel 9
        let s0 = r#"Mozilla/5.0 (Linux; Android 14; Pixel 9 Build/AD1A.240411.003.A5; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/124.0.6367.54 Mobile Safari/537.36"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile WebView", version: "124.0.6367.54", "#,
            r#"os: Some(Os { name: "Android", version: "14" }), "#,
            r#"device: "Google/Pixel 9" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);
    }
    #[test]
    fn test_user_agent_on_android_google_02() {
        // android: Google Pixel 8 Pro
        let s0 = r#"Mozilla/5.0 (Linux; Android 15; Pixel 8 Pro Build/AP4A.250105.002; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/132.0.6834.163 Mobile Safari/537.36"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile WebView", version: "132.0.6834.163", "#,
            r#"os: Some(Os { name: "Android", version: "15" }), "#,
            r#"device: "Google/Pixel 8 Pro" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);

        // android: Google Pixel 8
        let s0 = r#"Mozilla/5.0 (Linux; Android 15; Pixel 8 Build/AP4A.250105.002; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/132.0.6834.163 Mobile Safari/537.36"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile WebView", version: "132.0.6834.163", "#,
            r#"os: Some(Os { name: "Android", version: "15" }), "#,
            r#"device: "Google/Pixel 8" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);
    }
    #[test]
    fn test_user_agent_on_android_google_03() {
        // android: Google Pixel 7 Pro
        let s0 = r#"Mozilla/5.0 (Linux; Android 13; Pixel 7 Pro) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Mobile Safari/537.36"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile", version: "112.0.0.0", "#,
            r#"os: Some(Os { name: "Android", version: "13" }), "#,
            r#"device: "Google/Pixel 7 Pro" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);

        // android: Google Pixel 7
        let s0 = r#"Mozilla/5.0 (Linux; Android 13; Pixel 7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Mobile Safari/537.36"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile", version: "112.0.0.0", "#,
            r#"os: Some(Os { name: "Android", version: "13" }), "#,
            r#"device: "Google/Pixel 7" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);
    }
    #[test]
    fn test_user_agent_on_android_google_04() {
        // android: Google Pixel 6 Pro
        let s0 = r#"Mozilla/5.0 (Linux; Android 13; Pixel 6 Pro) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Mobile Safari/537.36"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile", version: "112.0.0.0", "#,
            r#"os: Some(Os { name: "Android", version: "13" }), "#,
            r#"device: "Google/Pixel 6 Pro" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);

        // android: Google Pixel 6a
        let s0 = r#"Mozilla/5.0 (Linux; Android 13; Pixel 6a) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Mobile Safari/537.36"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile", version: "112.0.0.0", "#,
            r#"os: Some(Os { name: "Android", version: "13" }), "#,
            r#"device: "Google/Pixel 6a" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);

        // android: Google Pixel 6
        let s0 = r#"Mozilla/5.0 (Linux; Android 13; Pixel 6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Mobile Safari/537.36"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile", version: "112.0.0.0", "#,
            r#"os: Some(Os { name: "Android", version: "13" }), "#,
            r#"device: "Google/Pixel 6" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);
    }
}

#[cfg(test)]
mod test_more_03 {
    use super::*;
    #[test]
    fn test_user_agent_on_android_motorola_01() {
        // android: Motorola Moto G (2025)
        let s0 = r#"Mozilla/5.0 (Linux; Android 15; moto g - 2025 Build/V1VK35.22-13-2; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/132.0.6834.163 Mobile Safari/537.36"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile WebView", version: "132.0.6834.163", "#,
            r#"os: Some(Os { name: "Android", version: "15" }), "#,
            r#"device: "Motorola/g - 2025" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);
        //break_dump_extractor!(s0);

        // android: Motorola Edge 30 Neo
        let s0 = r#"Dalvik/2.1.0 (Linux; U; Android 15; moto edge 30 neo Build/AP3A.241105.008)"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Android", version: "15", "#,
            r#"os: Some(Os { name: "Android", version: "15" }), "#,
            r#"device: "Motorola/edge 30 neo" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);
    }
    #[test]
    fn test_user_agent_on_android_motorola_02() {
        // android: Motorola Moto g04
        let s0 = r#"Mozilla/5.0 (Linux; Android 14; Moto g04) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.6261.64 Mobile Safari/537.36 Instabridge/22"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Instabridge", version: "22", "#,
            r#"os: Some(Os { name: "Android", version: "14" }), "#,
            r#"device: "Generic_Android/Moto g04" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);

        // android: Motorola G Stylus 5G (2024)
        let s0 =
            r#"Mozilla/5.0 (Linux; Android 14; moto g stylus 5G - 2024 Build/U2UB34.44-86; wv)"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Android", version: "14", "#,
            r#"os: Some(Os { name: "Android", version: "14" }), "#,
            r#"device: "Motorola/g stylus 5G - 2024" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);

        // android: Motorola G  Power 5G (2024)
        let s0 = r#"Mozilla/5.0 (Linux; Android 14; moto g power 5G - 2024 Build/U1UD34.16-62; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/123.0.6312.99 Mobile Safari/537.36"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile WebView", version: "123.0.6312.99", "#,
            r#"os: Some(Os { name: "Android", version: "14" }), "#,
            r#"device: "Motorola/g power 5G - 2024" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);

        // android: Motorola Razr 50 Ultra
        let s0 = r#"Mozilla/5.0 (Linux; Android 14; motorola razr 50 ultra Build/U3UX34.56-29-2; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/126.0.6478.134 Mobile Safari/537.36"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile WebView", version: "126.0.6478.134", "#,
            r#"os: Some(Os { name: "Android", version: "14" }), "#,
            r#"device: "Motorola/rola razr 50 ultra" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);
    }
    #[test]
    fn test_user_agent_on_android_motorola_03() {
        // android: Moto G Pure
        let s0 = r#"Mozilla/5.0 (Linux; Android 12; moto g pure) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Mobile Safari/537.36"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile", version: "112.0.0.0", "#,
            r#"os: Some(Os { name: "Android", version: "12" }), "#,
            r#"device: "Motorola/g pure" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);

        // android: Moto G Stylus 5G
        let s0 = r#"Mozilla/5.0 (Linux; Android 12; moto g stylus 5G) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Mobile Safari/537.36v"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile", version: "112.0.0.0", "#,
            r#"os: Some(Os { name: "Android", version: "12" }), "#,
            r#"device: "Motorola/g stylus 5G" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);

        // android: Moto G Stylus 5G (2022)
        let s0 = r#"Mozilla/5.0 (Linux; Android 12; moto g stylus 5G (2022)) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Mobile Safari/537.36"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile", version: "112.0.0.0", "#,
            r#"os: Some(Os { name: "Android", version: "12" }), "#,
            r#"device: "Motorola/g stylus 5G (2022)" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);

        // android: Moto G 5G (2022)
        let s0 = r#"Mozilla/5.0 (Linux; Android 12; moto g 5G (2022)) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Mobile Safari/537.36"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile", version: "112.0.0.0", "#,
            r#"os: Some(Os { name: "Android", version: "12" }), "#,
            r#"device: "Motorola/g 5G (2022)" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);

        // android: Moto G Power (2022)
        let s0 = r#"Mozilla/5.0 (Linux; Android 12; moto g power (2022)) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Mobile Safari/537.36"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile", version: "112.0.0.0", "#,
            r#"os: Some(Os { name: "Android", version: "12" }), "#,
            r#"device: "Motorola/g power (2022)" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);

        // android: Moto G Power (2021)
        let s0 = r#"Mozilla/5.0 (Linux; Android 11; moto g power (2021)) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Mobile Safari/537.36"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile", version: "112.0.0.0", "#,
            r#"os: Some(Os { name: "Android", version: "11" }), "#,
            r#"device: "Motorola/g power (2021)" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);
    }
}

#[cfg(test)]
mod test_more_04 {
    use super::*;
    #[test]
    fn test_user_agent_on_android_other_01() {
        // android: Redmi Note 13 4G
        let s0 = r#"Mozilla/5.0 (Linux; Android 13; 23129RAA4G Build/TKQ1.221114.001; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/116.0.0.0 Mobile Safari/537.36"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile WebView", version: "116.0.0.0", "#,
            r#"os: Some(Os { name: "Android", version: "13" }), "#,
            r#"device: "Generic_Android/23129RAA4G" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);

        // android: Redmi Turbo 4
        let s0 = r#"Mozilla/5.0 (Linux; Android 15; 24129RT7CC Build/AP3A.240905.015.A2; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/130.0.6723.86 Mobile Safari/537.36"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile WebView", version: "130.0.6723.86", "#,
            r#"os: Some(Os { name: "Android", version: "15" }), "#,
            r#"device: "Generic_Android/24129RT7CC" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);
    }
    #[test]
    fn test_user_agent_on_android_other_02() {
        // android: Huawei Pura 70 Ultra
        let s0 = r#"Mozilla/5.0 (Linux; Android 12; HBP-LX9 Build/HUAWEIHBP-L29; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/99.0.4844.88 Mobile Safari/537.36"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile WebView", version: "99.0.4844.88", "#,
            r#"os: Some(Os { name: "Android", version: "12" }), "#,
            r#"device: "Huawei/HBP-LX9" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);

        // android: Huawei Nova 12 Pro
        let s0 = r#"
        Mozilla/5.0 (Linux; U; Android 12; zh-Hans-CN; ADA-AL00 Build/HUAWEIADA-AL00) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/100.0.4896.58 Quark/6.11.2.531 Mobile Safari/537.36
            "#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile WebView", version: "100.0.4896.58", "#,
            r#"os: Some(Os { name: "Android", version: "12" }), "#,
            r#"device: "Huawei/ADA-AL00" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);

        // android: Huawei Nova Flip
        let s0 = r#"
        Mozilla/5.0 (Linux; Android 12; PSD-AL00 Build/HUAWEIPSD-AL00; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/99.0.4844.88 Mobile Safari/537.36
            "#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile WebView", version: "99.0.4844.88", "#,
            r#"os: Some(Os { name: "Android", version: "12" }), "#,
            r#"device: "Huawei/PSD-AL00" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);
    }
    #[test]
    fn test_user_agent_on_android_other_03() {
        // android: Xiaomi 14 Ultra
        let s0 = r#"Mozilla/5.0 (Linux; Android 14; 24030PN60G Build/UKQ1.231003.002; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/122.0.6261.119 Mobile Safari/537.36"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile WebView", version: "122.0.6261.119", "#,
            r#"os: Some(Os { name: "Android", version: "14" }), "#,
            r#"device: "Generic_Android/24030PN60G" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);

        // android: Mix Flip
        let s0 = r#"Mozilla/5.0 (Linux; Android 14; 2405CPX3DC Build/UKQ1.240116.001; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/120.0.6099.193 Mobile Safari/537.36"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile WebView", version: "120.0.6099.193", "#,
            r#"os: Some(Os { name: "Android", version: "14" }), "#,
            r#"device: "Generic_Android/2405CPX3DC" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);
    }
    #[test]
    fn test_user_agent_on_android_other_04() {
        // android: Redmi Note 9 Pro
        let s0 = r#"Mozilla/5.0 (Linux; Android 12; Redmi Note 9 Pro) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Mobile Safari/537.36"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile", version: "112.0.0.0", "#,
            r#"os: Some(Os { name: "Android", version: "12" }), "#,
            r#"device: "XiaoMi/Redmi Note 9 Pro" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);

        // android: Redmi Note 8 Pro
        let s0 = r#"Mozilla/5.0 (Linux; Android 11; Redmi Note 8 Pro) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Mobile Safari/537.36"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile", version: "112.0.0.0", "#,
            r#"os: Some(Os { name: "Android", version: "11" }), "#,
            r#"device: "XiaoMi/Redmi Note 8 Pro" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);
    }
    #[test]
    fn test_user_agent_on_android_other_05() {
        // android: Huawei P30 Pro
        let s0 = r#"Mozilla/5.0 (Linux; Android 10; VOG-L29) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Mobile Safari/537.36"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile", version: "112.0.0.0", "#,
            r#"os: Some(Os { name: "Android", version: "10" }), "#,
            r#"device: "Huawei/VOG-L29" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);

        // android: Huawei P30 lite
        let s0 = r#"Mozilla/5.0 (Linux; Android 10; MAR-LX1A) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Mobile Safari/537.36"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile", version: "112.0.0.0", "#,
            r#"os: Some(Os { name: "Android", version: "10" }), "#,
            r#"device: "Generic_Android/MAR-LX1A" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);
    }
    #[test]
    fn test_user_agent_on_android_other_06() {
        // android: Redmi Note 10 Pro
        let s0 = r#"Mozilla/5.0 (Linux; Android 13; M2101K6G) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Mobile Safari/537.36"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile", version: "112.0.0.0", "#,
            r#"os: Some(Os { name: "Android", version: "13" }), "#,
            r#"device: "Generic_Android/M2101K6G" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);

        // android: Xiaomi Poco X3 Pro
        let s0 = r#"Mozilla/5.0 (Linux; Android 12; M2102J20SG) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Mobile Safari/537.36"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile", version: "112.0.0.0", "#,
            r#"os: Some(Os { name: "Android", version: "12" }), "#,
            r#"device: "Generic_Android/M2102J20SG" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);

        // android: Redmi Note 11 Pro 5G
        let s0 = r#"
        Mozilla/5.0 (Linux; Android 12; 2201116SG) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Mobile Safari/537.36
            "#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile", version: "112.0.0.0", "#,
            r#"os: Some(Os { name: "Android", version: "12" }), "#,
            r#"device: "Generic_Android/2201116SG" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);

        // android: OnePlus Nord N200 5G
        let s0 = r#"
        Mozilla/5.0 (Linux; Android 12; DE2118) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Mobile Safari/537.36
            "#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Chrome Mobile", version: "112.0.0.0", "#,
            r#"os: Some(Os { name: "Android", version: "12" }), "#,
            r#"device: "OnePlus/OnePlus DE2118" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);
    }
}

#[cfg(test)]
mod test_more_05 {
    use super::*;
    #[test]
    fn test_user_agent_on_iphone_01() {
        // Apple iPhone 16e
        let s0 = r#"Mozilla/5.0 (iPhone17,5; CPU iPhone OS 18_3_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15E148 FireKeepers/1.7.0"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Mobile Safari UI/WKWebView", version: "", "#,
            r#"os: Some(Os { name: "iOS", version: "18.3.2" }), "#,
            r#"device: "Apple/iPhone17,5" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);

        // Apple iPhone 16 Pro
        let s0 = r#"Mozilla/5.0 (iPhone17,1; CPU iPhone OS 18_2_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15E148 Mohegan Sun/4.7.4"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Mobile Safari UI/WKWebView", version: "", "#,
            r#"os: Some(Os { name: "iOS", version: "18.2.1" }), "#,
            r#"device: "Apple/iPhone17,1" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);

        // Apple iPhone 16 Pro Max
        let s0 = r#"Mozilla/5.0 (iPhone17,2; CPU iPhone OS 18_3_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15E148 Resorts/4.5.2"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Mobile Safari UI/WKWebView", version: "", "#,
            r#"os: Some(Os { name: "iOS", version: "18.3.1" }), "#,
            r#"device: "Apple/iPhone17,2" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);

        // Apple iPhone 16
        let s0 = r#"Mozilla/5.0 (iPhone17,3; CPU iPhone OS 18_3_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15E148 FireKeepers/1.6.1"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Mobile Safari UI/WKWebView", version: "", "#,
            r#"os: Some(Os { name: "iOS", version: "18.3.2" }), "#,
            r#"device: "Apple/iPhone17,3" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);

        // Apple iPhone 16 Plus
        let s0 = r#"Mozilla/5.0 (iPhone17,4; CPU iPhone OS 18_2_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15E148 Resorts/4.7.5"#;
        let browser = convert_from_user_agent(s0).unwrap();
        let target = concat!(
            r#"Browser { name: "Mobile Safari UI/WKWebView", version: "", "#,
            r#"os: Some(Os { name: "iOS", version: "18.2.1" }), "#,
            r#"device: "Apple/iPhone17,4" }"#,
        );
        assert_eq!(format!("{browser:?}"), target);
    }
}
