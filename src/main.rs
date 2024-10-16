use gpconnect::gp::login::get_prelogin_cookie;
#[tokio::main]
async fn main() {
    let cookie = get_prelogin_cookie("http://127.0.0.1:5500/index.html").await.unwrap();
    println!("Cookie: {}", cookie);
}
