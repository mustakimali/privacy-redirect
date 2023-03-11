mod handlers;
mod middleware;
mod server;
mod tracing;

lazy_static::lazy_static! {
    pub(crate) static ref ALLOWED_LIST: Vec<&'static str> = vec![
        "okta.com", "aws.amazon.com", "amazonaws.com"
    ];
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing::init_tracing("privacy-redirect");

    server::start().await.expect("start server");

    Ok(())
}
