use actix_web::{
    http::{header, StatusCode},
    HttpResponse, Responder,
};
use serde_json::json;
use tracing::info;

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

const REDIRECT_HTML: &str = r#"
<!DOCTYPE html>
<html><head>
<title>Privacy Redirect</title>
<meta http-equiv="Refresh" content="0; url=$$URL$$" />
<meta name="referrer" content="no-referrer" />
<script type="text/javascript">
/* <![CDATA[ */
window.location.replace( "$$URL_ESCAPED$$" + window.location.hash );
/* ]]> */
</script>
</head>
<body style="background-color: black;color: white;"><p>Redirecting..<br /><a href="$$URL$$">$$URL$$</a></p></body></html>"#;
pub async fn redirect(req: actix_web::HttpRequest) -> impl Responder {
    let qs = req.query_string().to_string();
    let q = urlencoding::decode(qs.as_str())
        .map(|r| r.to_string())
        .unwrap_or_else(|_| qs);

    if !q.is_empty() {
        if let Ok(cleaned) = tracking_params::clean_str(&q) {
            info!(dirty = q, cleaned = cleaned, "Cleaned");

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
            let cleaned_escaped = cleaned.replace("/", r#"\/"#);
            let html = REDIRECT_HTML
                .replace("$$URL$$", &cleaned)
                .replace("$$URL_ESCAPED$$", &cleaned_escaped);
            return HttpResponse::Ok().body(html);
        }
    }

    // Redirect to Frontend App
    if cfg!(debug_assertions) {}

    HttpResponse::TemporaryRedirect()
        .append_header(("Location", "/app"))
        .finish()
}

pub async fn referrer(req: actix_web::HttpRequest) -> impl Responder {
    let r = match req
        .headers()
        .get(header::REFERER)
        .and_then(|r| r.to_str().ok())
        .map(|r| r.to_string())
    {
        Some(rfr) => HttpResponse::Ok().body(json!({ "referrer": rfr }).to_string()),
        None => HttpResponse::NotFound().finish(),
    };

    r
}
