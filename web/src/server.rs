use actix_web::{
    http::header::{HeaderName, HeaderValue},
    web::{self, ServiceConfig},
    App, HttpServer,
};
use actix_web_prometheus::PrometheusMetricsBuilder;
use chrono::Utc;
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
            .wrap(prometheus.clone())
            .wrap(actix_web::middleware::Compress::default())
            .wrap(super::tracing::PrivacyFriendlyTraceLogger::new())
            //.wrap(protect_endpoint_middleware::Midleware)
            .service(actix_files::Files::new("/app", "./static").index_file("index.html"))
            //.wrap(ProtectEndpoint)
            .configure(register_handlers)
    })
    .bind(addr)?
    .workers(4)
    .run()
    .await?;

    Ok(())
}

create_middleware!(
    ProtectEndpoint,
    {
        let mut req = req;

        let req_details = req.extract::<RequestDetails>();
        let path = req.path().to_string();
    },
    {
        let req_details = req_details.await?;
        let res = fut.await?;

        if path == "/metrics" && dbg!(req_details.ip_address).is_some() {
            return Err(super::handlers::HttpError::Forbidden.into());
        }
    }
);

// mod middleware {
//     use futures_util::future::LocalBoxFuture;
//     use std::future::{ready, Ready};

//     use actix_web::{
//         dev::{Service, ServiceRequest, ServiceResponse, Transform},
//         Error, FromRequest,
//     };

//     pub struct BlockRequestToMetricsMiddleware;
//     pub struct Timing<S> {
//         service: S,
//     }

//     impl<S, B> Transform<S, ServiceRequest> for BlockRequestToMetricsMiddleware
//     where
//         S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
//         S::Future: 'static,ProtectEndpoint
//         B: 'static,
//     {
//         type Response = ServiceResponse<B>;
//         type Error = Error;
//         type InitError = ();
//         type Transform = Timing<S>;
//         type Future = Ready<Result<Self::Transform, Self::InitError>>;

//         fn new_transform(&self, service: S) -> Self::Future {
//             ready(Ok(Timing { service }))
//         }
//     }

//     impl<S, B> Service<ServiceRequest> for Timing<S>
//     where
//         S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
//         S::Future: 'static,
//         B: 'static,
//     {
//         type Response = ServiceResponse<B>;

//         type Error = Error;

//         type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

//         fn poll_ready(
//             &self,
//             ctx: &mut core::task::Context<'_>,
//         ) -> std::task::Poll<Result<(), Self::Error>> {
//             self.service.poll_ready(ctx)
//         }

//         fn call(&self, req: ServiceRequest) -> Self::Future {
//             let mut req = req;

//             let req_details = req.extract::<RequestDetails>();
//             let path = req.path().to_string();
//             let fut = self.service.call(req);

//             Box::pin(async move {
//                 let req_details = req_details.await?;
//                 let res = fut.await?;

//                 if path == "/metrics" && dbg!(req_details.ip_address).is_some() {
//                     return Err(super::handlers::HttpError::Forbidden.into());
//                 }
//                 Ok(res)
//             })
//         }
//     }

//     pub struct RequestDetails {
//         pub ip_address: Option<String>,
//     }

//     impl FromRequest for RequestDetails {
//         type Error = Error;

//         type Future = Ready<Result<Self, Self::Error>>;

//         fn from_request(
//             req: &actix_web::HttpRequest,
//             _payload: &mut actix_web::dev::Payload,
//         ) -> Self::Future {
//             let ip_address = req
//                 .headers()
//                 .get("cf-connecting-ip")
//                 //.ok_or_else(|| PublicRequestError::Failed)
//                 .and_then(|ip| ip.to_str().ok())
//                 .map(|ip| ip.to_string());

//             ready(Ok(Self { ip_address }))
//         }
//     }
// }
