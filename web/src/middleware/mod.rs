#[macro_export]
macro_rules! create_middleware {
    ($name: ident, $code: expr, $head: tt, $tail: tt) => {
        paste::paste! {
            mod [<$name:snake _middleware>] {
                use futures_util::future::LocalBoxFuture;
                use std::future::{ready, Ready};

                use actix_web::{
                    dev::{Service, ServiceRequest, ServiceResponse, Transform},
                    Error, FromRequest,
                };

                pub struct Midleware;
                pub struct MiddlewareTransform<S> {
                    service: S,
                }

                impl<S, B> Transform<S, ServiceRequest> for Midleware
                where
                    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
                    S::Future: 'static,
                    B: 'static,
                {
                    type Response = ServiceResponse<B>;
                    type Error = Error;
                    type InitError = ();
                    type Transform = MiddlewareTransform<S>;
                    type Future = Ready<Result<Self::Transform, Self::InitError>>;

                    fn new_transform(&self, service: S) -> Self::Future {
                        ready(Ok(MiddlewareTransform { service }))
                    }
                }

                impl<S, B> Service<ServiceRequest> for MiddlewareTransform<S>
                where
                    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
                    S::Future: 'static,
                    B: 'static,
                {
                    type Response = ServiceResponse<B>;

                    type Error = Error;

                    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

                    fn poll_ready(
                        &self,
                        ctx: &mut core::task::Context<'_>,
                    ) -> std::task::Poll<Result<(), Self::Error>> {
                        self.service.poll_ready(ctx)
                    }

                    fn call(&self, req: ServiceRequest) -> Self::Future {
                        // let mut req = req;

                        // let req_details = req.extract::<RequestDetails>();
                        // let path = req.path().to_string();
                        // let fut = self.service.call(req);

                        $head

                        Box::pin(async move {
                            // let req_details = req_details.await?;
                            // let res = fut.await?;

                            // if path == "/metrics" && dbg!(req_details.ip_address).is_some() {
                            //     return Err(super::handlers::HttpError::Forbidden.into());
                            // }

                            $tail
                            Ok(res)
                        })
                    }
                }

                pub struct RequestDetails {
                    pub ip_address: Option<String>,
                }

                impl FromRequest for RequestDetails {
                    type Error = Error;

                    type Future = Ready<Result<Self, Self::Error>>;

                    fn from_request(
                        req: &actix_web::HttpRequest,
                        _payload: &mut actix_web::dev::Payload,
                    ) -> Self::Future {
                        let ip_address = req
                            .headers()
                            .get("cf-connecting-ip")
                            //.ok_or_else(|| PublicRequestError::Failed)
                            .and_then(|ip| ip.to_str().ok())
                            .map(|ip| ip.to_string());

                        ready(Ok(Self { ip_address }))
                    }
                }
            }
        }
    };
}
