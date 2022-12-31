use actix_web::{
    http::{header, StatusCode},
    HttpResponse, Responder,
};
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
<body style="background-color: black;"><p>Redirecting..<br /><a href="$$URL$$">$$URL$$</a></p></body></html>"#;

#[tracing::instrument(skip(req), fields(input_hash = "", cleaned = false, json = false))]
pub async fn redirect(req: actix_web::HttpRequest) -> impl Responder {
    let input_url = req.query_string().to_string();
    // let input_url = urlencoding::decode(&input_url)
    //     .map(|r| r.to_string())
    //     .unwrap_or_else(|_| qs);
    tracing::Span::current().record("input_hash", hash(&input_url));

    if !input_url.is_empty() {
        if let Ok(cleaned) = tracking_params::clean_str(&input_url) {
            let removed_trackers = cleaned != input_url;
            tracing::Span::current().record("cleaned", removed_trackers);

            if let Some(ct) = req.headers().get("content-type") {
                if ct
                    .as_bytes()
                    .windows(JSON_CONTENT_TYPE.len())
                    .any(|w| w.eq(JSON_CONTENT_TYPE))
                {
                    tracing::Span::current().record("json", true);
                    return HttpResponse::Ok()
                        .append_header(("content-type", "application/json"))
                        .body(
                            json!({
                                "cleaned_url": cleaned,
                                "original_url": input_url
                            })
                            .to_string(),
                        );
                }
            }
            let cleaned_escaped = cleaned.replace("/", r#"\/"#);
            let html = REDIRECT_HTML
                .replace("$$URL$$", &cleaned)
                .replace("$$URL_ESCAPED$$", &cleaned_escaped);
            return HttpResponse::Ok()
                .append_header(("cache-control", "public, max-age=86400"))
                .append_header(("vary", "*"))
                .body(html);
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

fn hash(input: &str) -> String {
    blake3::hash(input.as_bytes()).to_hex().to_string()
}
