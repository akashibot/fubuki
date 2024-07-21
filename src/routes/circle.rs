use std::sync::Arc;

use crate::utils::http::{ErrorResponse, ImagePayload, ImageResponse, IntoHttpResponse};
use actix_web::{get, HttpResponse};
use ril::{Ellipse, Image, L};

/// May fail be aware.
#[get("/circle")]
pub async fn circle(payload: ImagePayload) -> Result<HttpResponse, ErrorResponse> {
    let mut image = payload.image.clone();
    let (payload_width, payload_height) = image.dimensions();

    let ellipse = Ellipse::from_bounding_box(0, 0, payload_width, payload_height).with_fill(L(255));

    let mut mask = Image::new(payload_width, payload_height, L(0));
    mask.draw(&ellipse);
    image.mask_alpha(&mask);

    let response = Arc::new(ImageResponse::new(image, payload.format));

    response.into_http_response().await
}
