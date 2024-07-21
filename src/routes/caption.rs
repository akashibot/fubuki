use std::sync::Arc;

use crate::utils::http::{ErrorResponse, ImagePayload, ImageResponse, IntoHttpResponse};
use actix_web::{get, web, HttpResponse};
use ril::{Font, Image, Rgba, TextLayout, TextSegment, WrapStyle};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CaptionPayload {
    pub text: String,
}

#[get("/caption/{text}")]
pub async fn caption(
    payload: ImagePayload,
    caption_payload: web::Path<CaptionPayload>,
) -> Result<HttpResponse, ErrorResponse> {
    let text = caption_payload.into_inner().text;
    let (payload_width, payload_height) = payload.image.clone().dimensions();

    let mut caption_box = Image::new(payload_width, payload_height / 2, Rgba::white());
    let font_size = determine_font_size(&text, caption_box.width(), caption_box.height());
    let font = Font::open("./src/routes/fonts/caption.otf", font_size)?;

    let mut base = Image::<Rgba>::new(
        payload_width,
        payload_height + caption_box.height(),
        Rgba::transparent(),
    );

    let (caption_x, caption_y) = caption_box.center();
    let caption_layout = TextLayout::<Rgba>::new()
        .centered()
        .with_wrap(WrapStyle::Word)
        .with_width(base.width() - 15)
        .with_position(caption_x, caption_y)
        .with_segment(&TextSegment::new(&font, text, Rgba::black()));

    caption_box.draw(&caption_layout);

    base.paste(0, 0, &caption_box);
    base.paste(0, payload_height / 2, &payload.image.clone());

    let response = Arc::new(ImageResponse::new(base, payload.format));

    response.into_http_response().await
}

pub fn determine_font_size(text: &str, image_width: u32, image_height: u32) -> f32 {
    let text_length = text.chars().count();
    let font_size = 30.0;
    let text_height = font_size * 1.2;
    let text_width = font_size * 0.6 * text_length as f32;

    let max_font_size = f32::min(
        (image_width as f32 / text_width) * font_size,
        (image_height as f32 / text_height) * font_size,
    );

    max_font_size.round()
}
