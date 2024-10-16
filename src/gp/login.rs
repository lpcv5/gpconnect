use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use log::{error, info};
use reqwest::Client;
use serde::Deserialize;
use serde_xml_rs::from_str;
use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;

pub async fn get_login_url() -> Result<String, Box<dyn Error>> {
    let client = Client::builder()
        .danger_accept_invalid_certs(true) // This disables SSL certificate verification
        .build()?;
    let endpoint = "https://47.100.9.56/ssl-vpn/prelogin.esp";
    let params = [
        ("kerberos-support", "yes"),
        ("tmp", "tmp"),
        ("clientVer", "4100"),
        ("host-id", "19b792a1-abaf-49fc-9739-811c86099be1"),
        ("clientos", "Windows"),
        ("os-version", "Microsoft Windows 11 Pro , 64-bit"),
        ("ipv6-support", "yes"),
        ("default-browser", "0"),
        ("cas-support", "yes"),
    ];

    let res = client.post(endpoint).query(&params).send().await?;

    let xml: PreloginResponse = from_str(&res.text().await?)?;
    let status = xml.status.ok_or("Status not found")?;
    info!("Status: {}", status);

    let sr = xml.saml_request.ok_or("SAML request not found")?;
    let url = String::from_utf8(URL_SAFE.decode(&sr)?)?;

    Ok(url)
}

#[derive(Deserialize)]
struct PreloginResponse {
    status: Option<String>,
    #[serde(rename = "saml-request")]
    saml_request: Option<String>,
}

pub async fn get_prelogin_cookie(url: &str) -> Result<String, Box<dyn Error>> {
    // Open the browser and navigate to the specified URL
    unimplemented!()
}

// pub async fn perform_login(prelogin_cookie: &str) -> Result<String, Box<dyn Error>> {
//     let client = Client::new();
//     let url = "https://47.100.9.56/ssl-vpn/login.esp";
//     let cookies = [("CLIENTOS", "V2luZG93cw==")];

//     let data = [
//         ("prot", "https:"),
//         ("server", "47.100.9.56"),
//         ("inputStr", ""),
//         ("jnlpReady", "jnlpReady"),
//         ("user", "ling.pcheng@fujitsu.com"),
//         ("passwd", ""),
//         ("computer", "G08CNXDFXSLPCH"),
//         ("ok", "Login"),
//         ("direct", "yes"),
//         ("clientVer", "4100"),
//         ("os-version", "Microsoft Windows 11 Pro , 64-bit"),
//         ("preferred-ip", ""),
//         ("preferred-ipv6", ""),
//         ("clientos", "Windows"),
//         ("clientgpversion", "6.0.8-601"),
//         ("portal-userauthcookie", "VwI5qlDKwkIv9HLXYYYogEusK9RZYjbIOBSW+7ppb1DpImDm5q5Xv/Thc1i+xFe3FaZND5uvJgoO93HfL0LJB3vGrheQCt+BJMNC7faljhlhecwEfVa3MvgvEWZZ2Xh7tTfoOV0bCSEhfjGJ2MBBMjUqdpYE5G7G8PIl8PM3AmqzcvGzMCoZYDWMSed1OiMJrmdf+FEQLbeacDXDq2NS8lr0PUtwlDWwVPRjomWhhj27/TiUeXKR3rxVRonGS9LBc4O18jIxZi6VDTgMyRI8bToPTGpfC3oNcYQrbnn/qAqcK28/WjInjOPGSEkcssLMWAo9YO2CgW47cjHv+kMyjA=="),
//         ("portal-prelogonuserauthcookie", "empty"),
//         ("host-id", "19b792a1-abaf-49fc-9739-811c86099be1"),
//         ("prelogin-cookie", prelogin_cookie),
//         ("ipv6-support", "yes"),
//         ("client-ip", "127.0.0.1"),
//         ("client-ipv6", ""),
//         ("internal", "no"),
//         ("serialno", "DZVQPM3"),
//         ("connect-method", "user-logon"),
//         ("selection-type", "auto"),
//         ("token", ""),
//         ("host", "47.100.9.56"),
//         ("gw", "CN-ALISH-GW"),
//     ];

//     let res = client.post(url)
//         .cookies(&cookies)
//         .form(&data)
//         .send()
//         .await?;

//     info!("Status Code: {}", res.status());
//     info!("Response Headers: {:?}", res.headers());
//     info!("Response Content: {}", res.text().await?);

//     Ok(res.text().await?)
// }

// pub async fn get_config(auth_cookie: &str) -> Result<String, Box<dyn Error>> {
//     let client = Client::new();
//     let url = "https://47.100.9.56/ssl-vpn/getconfig.esp";
//     let cookies = [("CLIENTOS", "V2luZG93cw==")];

//     let data = [
//         ("user", "ling.pcheng@fujitsu.com"),
//         ("addr1", "172.26.112.1/20"),
//         ("addr2", "172.16.200.227/22"),
//         ("preferred-ip", ""),
//         ("preferred-ipv6", ""),
//         ("portal", "GP-GW-SHAP-N"),
//         ("authcookie", auth_cookie),
//         ("client-type", "1"),
//         ("exclude-video-support", "yes"),
//         ("os-version", "Microsoft Windows 11 Pro , 64-bit"),
//         ("app-version", "6.0.8-601"),
//         ("protocol-version", "p1"),
//         ("clientos", "Windows"),
//         ("ipv6-support", "yes"),
//         ("internal", "no"),
//         ("client-ip", "172.16.200.227"),
//         ("client-ipv6", ""),
//         ("serialno", "DZVQPM3"),
//         ("mac-addr", "f2-e3-f7-1c-85-b3"),
//         ("joined-domain", "g08.fujitsu.local"),
//         ("enc-algo", "aes-256-gcm,aes-128-gcm,aes-128-cbc,"),
//         ("hmac-algo", "sha1,"),
//     ];

//     let res = client.post(url)
//         .cookies(&cookies)
//         .form(&data)
//         .send()
//         .await?;

//     info!("Config Status Code: {}", res.status());
//     info!("Config Response Headers: {:?}", res.headers());
//     info!("Config Response Content: {}", res.text().await?);

//     Ok(res.text().await?)
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_login_url() {
        let mut loginurl = String::new();
        match get_login_url().await {
            Ok(url) => loginurl = url,
            Err(e) => {
                panic!("Error: {}", e)
            }
        }
        match get_prelogin_cookie(&loginurl).await {
            Ok(cookie) => {
                println!("Prelogin Cookie: {}", cookie);
            }
            Err(e) => {
                panic!("Error: {}", e)
            }
        }
    }
}
