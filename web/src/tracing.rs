use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::Error;
use tracing::Span;
use tracing_actix_web::{DefaultRootSpanBuilder, RootSpanBuilder};

pub fn init_tracing(service_name: &str) {
    use opentelemetry_otlp::WithExportConfig;
    use std::collections::HashMap;
    use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
    use tracing_log::LogTracer;
    use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

    let env_filter = EnvFilter::new("info");
    let formatting_layer = BunyanFormattingLayer::new("privacy-redirect".into(), std::io::stdout);

    LogTracer::init().expect("Failed to set logger");

    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    if let Ok(api_key) = std::env::var("HONEYCOMB_API_KEY") {
        std::env::set_var("OTEL_SERVICE_NAME", service_name);

        let tracer = opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(
                opentelemetry_otlp::new_exporter()
                    .http()
                    .with_endpoint("https://api.honeycomb.io/v1/traces")
                    .with_http_client(reqwest::Client::default())
                    .with_headers(HashMap::from([
                        ("x-honeycomb-dataset".into(), service_name.into()),
                        ("x-honeycomb-team".into(), api_key.into()),
                    ]))
                    .with_timeout(std::time::Duration::from_secs(2)),
            ) // Replace with runtime::Tokio if using async main
            .install_batch(opentelemetry::runtime::TokioCurrentThread)
            .expect("install");
        let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
        let subscriber = subscriber.with(telemetry);
        tracing::subscriber::set_global_default(subscriber).unwrap();
    } else {
        tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");
        tracing::info!("Tracing initialized without Honeycomb exporter");
    }
}

/// A Trace Logger that replaces the field `http.target` with its hash
/// so that the server doesn't log any query string.
pub type PrivacyFriendlyTraceLogger =
    tracing_actix_web::TracingLogger<PrivacyFriendlyRootSpanBuilder>;

pub struct PrivacyFriendlyRootSpanBuilder;

impl RootSpanBuilder for PrivacyFriendlyRootSpanBuilder {
    fn on_request_start(request: &ServiceRequest) -> Span {
        let target = request
            .uri()
            .path_and_query()
            .map(|p| p.as_str())
            .unwrap_or("");
        tracing_actix_web::root_span!(request, http.target = hash(target))
    }

    fn on_request_end<B: MessageBody>(span: Span, response: &Result<ServiceResponse<B>, Error>) {
        DefaultRootSpanBuilder::on_request_end(span, response);
    }
}

pub fn hash(input: &str) -> String {
    blake3::hash(input.as_bytes()).to_hex().to_string()
}
