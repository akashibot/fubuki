use crate::utils::http::{ErrorResponse, ImagePayload, ImageResponse};
use actix_web::{get, web, HttpResponse};
use ril::ImageFormat;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ConvertPayload {
    pub mime: String,
}

#[get("/convert/{mime}")]
pub async fn convert(
    payload: ImagePayload,
    convert_payload: web::Path<ConvertPayload>,
) -> Result<HttpResponse, ErrorResponse> {
    let mime = convert_payload.mime.clone();
    let mime = if mime == "jpg" { "jpeg" } else { &mime };

    let new_mime = ImageFormat::from_extension(mime).unwrap_or(payload.format);

    match new_mime {
        ImageFormat::Unknown => Err(ErrorResponse {
            message: "unknown mime".to_string(),
        }),
        _ => ImageResponse {
            data: payload.image.clone(),
            format: new_mime,
        }
        .try_into(),
    }
}
