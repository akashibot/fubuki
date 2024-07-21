use std::sync::Arc;

use crate::utils::http::{ErrorResponse, ImagePayload, ImageResponse, IntoHttpResponse};
use actix_web::{get, HttpResponse};

#[get("/invert")]
pub async fn invert(payload: ImagePayload) -> Result<HttpResponse, ErrorResponse> {
    let response = Arc::new(ImageResponse::new(!payload.image.clone(), payload.format));

    response.into_http_response().await
}
