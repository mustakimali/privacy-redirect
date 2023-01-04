mod handlers;
mod server;

use tracing::{info, subscriber::set_global_default};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, Registry};

lazy_static::lazy_static! {
    pub(crate) static ref ALLOWED_LIST: Vec<&'static str> = vec![
        "okta.com", "aws.amazon.com"
    ];
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_tracing();

    server::start().await.expect("start server");

    Ok(())
}

fn init_tracing() {
    use tracing_subscriber::layer::SubscriberExt;

    let env_filter = EnvFilter::new("info");
    let formatting_layer = BunyanFormattingLayer::new("privacy-redirect".into(), std::io::stdout);

    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    LogTracer::init().expect("Failed to set logger");

    if let Ok(api_key) = std::env::var("HONEYCOMB_API_KEY") {
        let honey = tracing_honeycomb::new_honeycomb_telemetry_layer(
            "privacy-redirect",
            libhoney::Config {
                options: libhoney::client::Options {
                    api_key,
                    dataset: "privacy-redirect".to_string(),
                    ..Default::default()
                },
                transmission_options: libhoney::transmission::Options::default(),
            },
        );

        set_global_default(subscriber.with(honey)).expect("Failed to set subscriber");
    } else {
        set_global_default(subscriber).expect("Failed to set subscriber");
        info!("Tracing initialized without Honeycomb exporter")
    };
}
