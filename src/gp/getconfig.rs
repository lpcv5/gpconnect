use crate::core::config::Config;
use reqwest::cookie::Jar;
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use serde_xml_rs::from_str;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    #[serde(rename = "gw-address")]
    gw_address: String,
    #[serde(rename = "ip-address")]
    ip_address: String,
    ipsec: Ipsec,
}

#[derive(Debug, Serialize, Deserialize)]
struct Ipsec {
    #[serde(rename = "c2s-spi")]
    c2s_spi: String,
    #[serde(rename = "s2c-spi")]
    s2c_spi: String,
    #[serde(rename = "akey-s2c")]
    akey_s2c: Key,
    #[serde(rename = "ekey-s2c")]
    ekey_s2c: Key,
    #[serde(rename = "akey-c2s")]
    akey_c2s: Key,
    #[serde(rename = "ekey-c2s")]
    ekey_c2s: Key,
}

#[derive(Debug, Serialize, Deserialize)]
struct Key {
    bits: String,
    val: String,
}

async fn get_config(auth_cookie: &str, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://47.100.9.56/ssl-vpn/getconfig.esp";

    let jar = Jar::default();
    jar.add_cookie_str("CLIENTOS=V2luZG93cw==", &url.parse::<Url>().unwrap());
    let client = Client::builder()
        .cookie_provider(Arc::new(jar))
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let mut data = HashMap::new();
    data.insert("user", "ling.pcheng@fujitsu.com");
    data.insert("addr1", "172.26.112.1/20");
    data.insert("addr2", "172.16.200.227/22");
    data.insert("preferred-ip", "");
    data.insert("preferred-ipv6", "");
    data.insert("portal", "GP-GW-SHAP-N");
    data.insert("authcookie", auth_cookie);
    data.insert("client-type", "1");
    data.insert("exclude-video-support", "yes");
    data.insert("os-version", "Microsoft Windows 11 Pro , 64-bit");
    data.insert("app-version", "6.0.8-601");
    data.insert("protocol-version", "p1");
    data.insert("clientos", "Windows");
    data.insert("ipv6-support", "yes");
    data.insert("internal", "no");
    data.insert("client-ip", "172.16.200.227");
    data.insert("client-ipv6", "");
    data.insert("serialno", "DZVQPM3");
    data.insert("mac-addr", "f2-e3-f7-1c-85-b3");
    data.insert("joined-domain", "g08.fujitsu.local");
    data.insert("enc-algo", "aes-256-gcm,aes-128-gcm,aes-128-cbc,");
    data.insert("hmac-algo", "sha1,");

    match client.post(url).form(&data).send().await {
        Ok(response) => match response.text().await {
            Ok(text) => {
                let response: Response = from_str(&text)?;
                Ok(())
            }
            Err(e) => Err(Box::new(e)),
        },
        Err(e) => Err(Box::new(e)),
    }
}

fn parse_config(config: Response) -> Result<Config, Box<dyn std::error::Error>> {

}
2024-10-18 17:20:29,411 - DEBUG - preferred_ip: None
2024-10-18 17:20:29,411 - DEBUG - portal_userauthcookie: AgFAGUDXuLHFHEIpWns6FoIg1elk7VX7r3eAiTakCEmksbf0AEQECkdmuskDhSc/GLV2bA06dd1ZXQYyGjJG2XL/iMlJCETTZTi6W72yCLCqXbcW46szeXbnGHUKLySuwxXEV5Al4/fLXskL5ebBxaaGksvDgMz+7xLMc0ytMPcQ32tDhxDfFTxxDI+hWCMAethHcZ/Wav36sx3na3J43lfGfd6ZgV2u4kz33rFqIzKx/qcaSLOaGInMPx9iUK8iR/NCw+GIvunKD9f4P3Nwgd8wd76LjClq7CRdNiFngwpCGAoOuBJ6cVAU+fds3Xaju3lqQhRO4bplzYdpCF4LFA==
2024-10-18 17:20:29,412 - DEBUG - portal_prelogonuserauthcookie: YrAUhbp/iKji8ucbQbqppmhDcZtakeqf6ZTNYTId3WL9wlXpT/jffgiPu1zq4N5eKmkLz+WO5Nsyj4scaOiaXQIHK1t9MzpiAVJeqoRQVXs03VtwQNecm3Zb4sjO/XKw64OQRvJ2fCXq/gmEBoiU5nrvzTy/IzS3VtIVZPKUmI+9w8Tx7WmOD9+a+AiQvsAFlXamFGOkb7QAAIT+4KXLEbQRURyEI/+FXP7bWZ0liEXCX9zKHQ/zxPuNzVYFhVulqOrdbL/AlGSzPAP1LS01aYyt+FGPyPCt7xY4R1l9TOpjpSQrugKr41JAMNfytrhcCH+MvC3Ym+LLmrpAQXMXnQ==
2024-10-18 17:20:29,413 - DEBUG - ipv6: None