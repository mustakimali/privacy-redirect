use actix_web::{
    http::header::{HeaderName, HeaderValue},
    web::{self, ServiceConfig},
    App, HttpServer,
};
use chrono::Utc;
use tracing::info;

use crate::handlers;

pub(crate) async fn start() -> anyhow::Result<()> {
    let addr = if cfg!(debug_assertions) {
        "127.0.0.1:8080"
    } else {
        "0.0.0.0:8080"
    }
    .to_string();

    start_inner(addr).await?;

    Ok(())
}

fn register_handlers(cfg: &mut ServiceConfig) {
    cfg.route("/", web::get().to(handlers::redirect))
        .route("/what-is-my-referrer", web::get().to(handlers::referrer))
        .route("/api/v1/healthcheck", web::get().to(handlers::health));
}

async fn start_inner(addr: String) -> anyhow::Result<()> {
    info!("Starting server on {}", addr);

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
            .service(actix_files::Files::new("/app", "./static").index_file("index.html"))
            .configure(register_handlers)
    })
    .bind(addr)?
    .workers(4)
    .run()
    .await?;

    Ok(())
}
