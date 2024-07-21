use std::sync::Arc;

use actix_web::{get, web, HttpResponse};
use serde::Deserialize;

use crate::utils::http::{ErrorResponse, ImagePayload, ImageResponse, IntoHttpResponse};

#[derive(Deserialize)]
pub struct OpacityPayload {
    pub value: f32,
}

#[get("/opacity/{value}")]
pub async fn opacity(
    payload: ImagePayload,
    opacity_payload: web::Path<OpacityPayload>,
) -> Result<HttpResponse, ErrorResponse> {
    let value = opacity_payload.value;

    if !(0.0..=1.0).contains(&value) {
        return Err(ErrorResponse {
            message: "opacity must be between 0 and 1".to_string(),
        });
    }

    let image = payload.image.clone();

    let result = image.map_pixels(|pixel| {
        let mut rgba = pixel;
        rgba.a = (rgba.a as f32 * value) as u8;
        rgba
    });

    let response = Arc::new(ImageResponse::new(result, payload.format));

    response.into_http_response().await
}
