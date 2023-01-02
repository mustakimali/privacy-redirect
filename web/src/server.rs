use actix_web::{
    http::header::{HeaderName, HeaderValue},
    web::{self, ServiceConfig},
    App, HttpServer,
};
use actix_web_opentelemetry::{PrometheusMetricsHandler, RequestMetricsBuilder, RequestTracing};
use chrono::Utc;
use opentelemetry::sdk::{
    export::metrics::aggregation,
    metrics::{controllers, processors, selectors},
};
use tracing::info;

use crate::handlers;

pub(crate) async fn start() -> anyhow::Result<()> {
    let addr = if cfg!(debug_assertions) {
        "127.0.0.1:8080"
    } else {
        "0.0.0.0:8080"
    }
    .to_string();

    start_server(addr).await?;

    Ok(())
}

fn register_handlers(cfg: &mut ServiceConfig) {
    cfg.route("/", web::get().to(handlers::redirect))
        .route("/api/v1/healthcheck", web::get().to(handlers::health));
}

async fn start_server(addr: String) -> anyhow::Result<()> {
    info!("Starting server on {}", addr);

    let (metrics_handler, request_metrics) = init_metrics();

    HttpServer::new(move || {
        App::new()
            .wrap_fn(|req, srv| {
                let start = Utc::now();

                let fut = actix_web::dev::Service::call(&srv, req);
                async move {
                    let mut res = fut.await?;
                    let duration = Utc::now() - start;
                    res.headers_mut().insert(
                        HeaderName::from_static("x-app-time-ms"),
                        HeaderValue::from_str(&format!("{}", duration.num_milliseconds()))?,
                    );
                    res.headers_mut().insert(
                        HeaderName::from_static("x-app-time-micros"),
                        HeaderValue::from_str(&format!(
                            "{}",
                            duration.num_microseconds().unwrap_or_default()
                        ))?,
                    );
                    // CORS header
                    res.headers_mut().insert(
                        HeaderName::from_static("access-control-allow-origin"),
                        HeaderValue::from_str("*")?,
                    );
                    res.headers_mut().insert(
                        HeaderName::from_static("access-control-allow-methods"),
                        HeaderValue::from_str("GET, POST, OPTIONS")?,
                    );
                    Ok(res)
                }
            })
            .wrap(actix_web::middleware::Compress::default())
            .route("/metrics", web::get().to(metrics_handler.clone()))
            .wrap(RequestTracing::new())
            .wrap(request_metrics.clone())
            .service(actix_files::Files::new("/app", "./static").index_file("index.html"))
            .configure(register_handlers)
    })
    .bind(addr)?
    .workers(4)
    .run()
    .await?;

    Ok(())
}

fn init_metrics() -> (
    PrometheusMetricsHandler,
    actix_web_opentelemetry::RequestMetrics,
) {
    let metrics_handler = {
        let controller = controllers::basic(
            processors::factory(
                selectors::simple::histogram([1.0, 2.0, 5.0, 10.0, 20.0, 50.0]),
                aggregation::cumulative_temporality_selector(),
            )
            .with_memory(true),
        )
        .build();

        let exporter = opentelemetry_prometheus::exporter(controller).init();
        PrometheusMetricsHandler::new(exporter)
    };
    let meter = opentelemetry::global::meter("actix_web");
    let request_metrics = RequestMetricsBuilder::new().build(meter);
    (metrics_handler, request_metrics)
}
