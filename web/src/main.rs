mod handlers;
mod server;

use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, EnvFilter, Registry};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_tracing();

    server::start().await.expect("start server");

    Ok(())
}

fn init_tracing() {
    let env_filter = EnvFilter::new("info".to_string());
    let formatting_layer = BunyanFormattingLayer::new("privacy-press".into(), std::io::stdout);

    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}
