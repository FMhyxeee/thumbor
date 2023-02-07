use axum::{extract::Path, routing::get, Router};
use percent_encoding::percent_decode_str;
use reqwest::StatusCode;
use serde::Deserialize;

pub use thumbor::ImageSpec;

#[derive(Deserialize)]
struct Params {
    spec: String,
    url: String,
}

#[tokio::main]
async fn main() {
    // 初始化 tracing
    tracing_subscriber::fmt::init();

    // 构建路由
    let app = Router::new().route("/image/:spec/:url", get(generate));

    let addr = "127.0.0.1:3000".parse::<std::net::SocketAddr>().unwrap();

    tracing::debug!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn generate(Path(Params { spec, url }): Path<Params>) -> Result<String, StatusCode> {
    let url = percent_decode_str(&url).decode_utf8_lossy();
    let spec: ImageSpec = spec
        .as_str()
        .try_into()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    Ok(format!("url: {url}\n spec: {spec:#?}"))
}
