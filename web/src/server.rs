use actix_web::{
    web::{self, ServiceConfig},
    App, HttpServer,
};
use actix_web_prometheus::PrometheusMetricsBuilder;
use tracing::info;

use crate::{create_middleware, handlers};

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
        .route(
            "/api/v1/allowed-list",
            web::get().to(handlers::allowed_list),
        )
        .route("/api/v1/healthcheck", web::get().to(handlers::health));
}

async fn start_server(addr: String) -> anyhow::Result<()> {
    info!("Starting server on {}", addr);

    //let (metrics_handler, request_metrics) = init_metrics();
    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .build()
        .expect("build prometheus builderal");

    HttpServer::new(move || {
        App::new()
            .wrap(super::tracing::PrivacyFriendlyTraceLogger::new())
            .wrap(protect_endpoint_middleware::Middleware)
            .wrap(timing_cors_headers_middleware::Middleware)
            .wrap(prometheus.clone())
            .service(actix_files::Files::new("/app", "./frontend").index_file("index.html"))
            //.wrap(ProtectEndpoint)
            .configure(register_handlers)
    })
    .bind(addr)?
    .workers(4)
    .run()
    .await?;

    Ok(())
}

pub struct RequestDetails {
    pub ip_address: Option<String>,
}

impl actix_web::FromRequest for RequestDetails {
    type Error = actix_web::Error;

    type Future = futures_util::future::Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let ip_address = req
            .headers()
            .get("cf-connecting-ip")
            .and_then(|ip| ip.to_str().ok())
            .map(|ip| ip.to_string());

        futures_util::future::ready(Ok(Self { ip_address }))
    }
}

create_middleware!(
    ProtectEndpoint,
    |ctx: &MiddlewareTransform<S>, req: ServiceRequest| {
        let mut req = req;

        let req_details = req.extract::<super::RequestDetails>();
        let path = req.path().to_string();
        let fut = ctx.service.call(req);

        Box::pin(async move {
            let req_details = req_details.await?;
            let res = fut.await?;

            if path == "/metrics" && req_details.ip_address.is_some() {
                return Err(super::handlers::HttpError::Forbidden.into());
            }

            Ok(res)
        })
    }
);

create_middleware!(
    TimingCorsHeaders,
    |ctx: &MiddlewareTransform<S>, req: ServiceRequest| {
        use actix_web::http::header::{HeaderName, HeaderValue};
        use chrono::Utc;

        let start = Utc::now();

        let fut = ctx.service.call(req);
        Box::pin(async move {
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
        })
    }
);
