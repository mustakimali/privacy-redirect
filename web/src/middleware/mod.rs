#[macro_export]
macro_rules! create_middleware {
    ($name: ident, $code: expr) => {
        paste::paste! {
            mod [<$name:snake _middleware>] {
                use futures_util::future::LocalBoxFuture;
                use std::future::{ready, Ready};

                use actix_web::{
                    dev::{Service, ServiceRequest, ServiceResponse, Transform},
                    Error
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
                        $code(self, req)
                    }
                }
            }
        }
    };
}
