use crate::utils::{
    self,
    http::{ErrorResponse, ImagePayload},
};
use actix_web::{
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    web, Error, HttpMessage,
};
use futures_util::future::LocalBoxFuture;
use std::future::ready;
use std::{future::Ready, sync::Arc};

pub struct ImageParser;

impl<S> Transform<S, ServiceRequest> for ImageParser
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Transform = ImageParserMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ImageParserMiddleware {
            service: Arc::new(service),
        }))
    }
}

pub struct ImageParserMiddleware<S> {
    service: Arc<S>,
}

impl<S> Service<ServiceRequest> for ImageParserMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = Arc::clone(&self.service);

        Box::pin(async move {
            // Retrieve and process query parameters
            let image_url = web::Query::<utils::http::ImageSource>::from_query(req.query_string());

            if let Ok(image_url) = image_url {
                // Fetch image and update request extensions
                match ImagePayload::from_url(&image_url.url).await {
                    Ok(payload) => {
                        req.extensions_mut().insert(payload);
                    }
                    Err(e) => {
                        // Handle errors and return an appropriate response
                        let error_response = ErrorResponse {
                            message: e.to_string(),
                        };
                        return Ok(req.error_response(error_response));
                    }
                }
            }

            // Call the downstream service
            let res = svc.call(req).await?;

            Ok(res)
        })
    }
}
