use actix_web::{http::StatusCode, HttpResponse, Responder};
use serde_json::json;

#[derive(thiserror::Error, Debug)]
pub enum HttpError {
    #[error("Internal Server Error")]
    InternalServerError(#[from] anyhow::Error),
}

impl actix_web::ResponseError for HttpError {
    fn status_code(&self) -> StatusCode {
        match self {
            HttpError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub async fn health() -> impl Responder {
    HttpResponse::Ok().body(
        json!({
            "status": "ok"
        })
        .to_string(),
    )
}

const JSON_CONTENT_TYPE: &[u8] = "application/json".as_bytes();
pub async fn redirect(req: actix_web::HttpRequest) -> impl Responder {
    let q = req.query_string();
    if !q.is_empty() {
        if let Ok(cleaned) = tracking_params::clean_str(q) {
            if let Some(ct) = req.headers().get("content-type") {
                if ct
                    .as_bytes()
                    .windows(JSON_CONTENT_TYPE.len())
                    .any(|w| w.eq(JSON_CONTENT_TYPE))
                {
                    return HttpResponse::Ok()
                        .append_header(("content-type", "application/json"))
                        .body(
                            json!({
                                "cleaned_url": cleaned,
                                "original_url": q
                            })
                            .to_string(),
                        );
                }
            }
            return HttpResponse::TemporaryRedirect()
                .append_header(("Location", cleaned.as_str()))
                .finish();
        }
    }

    // Redirect to Frontend App
    if cfg!(debug_assertions) {}

    HttpResponse::TemporaryRedirect()
        .append_header(("Location", "/app"))
        .finish()
}
