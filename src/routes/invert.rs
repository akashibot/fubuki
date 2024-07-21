use crate::utils::http::{ErrorResponse, ImagePayload, ImageResponse};
use actix_web::{get, HttpResponse};

#[get("/invert")]
pub async fn invert(payload: ImagePayload) -> Result<HttpResponse, ErrorResponse> {
    ImageResponse {
        data: !payload.image.clone(),
        format: payload.format,
    }
    .try_into()
}
