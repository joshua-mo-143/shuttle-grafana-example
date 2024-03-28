use axum::{routing::get, Router};
use base64::prelude::*;
use shuttle_runtime::SecretStore;
use std::process;
use tracing_subscriber::filter::{EnvFilter, LevelFilter};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use url::Url;

#[tracing::instrument]
async fn hello_world() -> &'static str {
    tracing::info!("An event happened!");
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secrets: SecretStore) -> shuttle_axum::ShuttleAxum {
    setup_tracing(&secrets);

    let router = Router::new().route("/", get(hello_world));

    Ok(router.into())
}

fn setup_tracing(secrets: &SecretStore) {
    let url = Url::parse("https://logs-prod-012.grafana.net").expect("Failed to parse Grafana URL");

    let grafana_user = secrets.get("GRAFANA_USER").unwrap();
    let grafana_password = secrets.get("GRAFANA_API_KEY").unwrap();

    let basic_auth = format!("{grafana_user}:{grafana_password}");
    let encoded_basic_auth = BASE64_STANDARD.encode(basic_auth.as_bytes());

    let (layer, task) = tracing_loki::builder()
        .label("application", "shuttle-grafana")
        .unwrap()
        .extra_field("pid", format!("{}", process::id()))
        .unwrap()
        .http_header("Authorization", format!("Basic {encoded_basic_auth}"))
        .unwrap()
        .build_url(url)
        .unwrap();

    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::DEBUG.into())
        .parse("")
        .unwrap();

    // We need to register our layer with `tracing`.
    tracing_subscriber::registry()
        .with(filter)
        .with(layer)
        .with(tracing_subscriber::fmt::Layer::new())
        // One could add more layers here, for example logging to stdout:
        // .with(tracing_subscriber::fmt::Layer::new())
        .init();

    // The background task needs to be spawned so the logs actually get
    // delivered.
    tokio::spawn(task);
}
