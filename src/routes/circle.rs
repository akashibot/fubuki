use crate::utils::http::{ErrorResponse, ImagePayload, ImageResponse};
use actix_web::{get, HttpResponse};
use ril::{Ellipse, Image, L};

/// May fail be aware.
#[get("/circle")]
pub async fn circle(payload: ImagePayload) -> Result<HttpResponse, ErrorResponse> {
    let mut image = payload.image.clone();
    let (payload_width, payload_height) = image.dimensions();

    let ellipse =
        Ellipse::from_bounding_box(0, 0, payload_width, payload_height).with_fill(L(255));

    let mut mask = Image::new(payload_width, payload_height, L(0));
    mask.draw(&ellipse);
    image.mask_alpha(&mask);
    
    ImageResponse {
        data: image,
        format: payload.format,
    }
        .try_into()
}
