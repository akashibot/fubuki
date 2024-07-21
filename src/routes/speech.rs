use std::sync::Arc;

use crate::utils::http::{ErrorResponse, ImagePayload, ImageResponse, IntoHttpResponse};
use actix_web::{get, HttpResponse};
use ril::{Image, Rgba};

#[get("/speech")]
pub async fn speech(payload: ImagePayload) -> Result<HttpResponse, ErrorResponse> {
    let mut balloon =
        Image::<Rgba>::open("./src/routes/images/speech.png").map_err(ErrorResponse::from)?;

    let (payload_width, payload_height) = payload.image.clone().dimensions();

    balloon.resize(payload_width, payload_height, ril::ResizeAlgorithm::Nearest);

    let mut base = Image::<Rgba>::new(payload_width, payload_height * 2, Rgba::black());

    base.paste(0, 0, &balloon);
    base.paste(0, payload_height, &payload.image.clone());

    let response = Arc::new(ImageResponse::new(base, payload.format));

    response.into_http_response().await
}
