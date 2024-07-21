use actix_web::{
    body::BoxBody,
    error,
    http::header::{CacheControl, CacheDirective, ContentType, /* ETag, EntityTag, IfNoneMatch */},
    FromRequest, HttpMessage, HttpResponse,
};
use reqwest::Client;
use ril::{Image, ImageFormat, Rgba};
use serde::{Deserialize, Serialize};
use std::{
    future::{ready, Ready},
    sync::{Arc, Mutex},
};
use tokio::task::JoinError;
// use md5;

#[derive(Deserialize, Debug, Clone)]
pub struct ImageSource {
    pub url: String,
}

#[derive(Clone)]
pub struct ImagePayload {
    pub image: Image<Rgba>,
    pub format: ImageFormat,
}

impl ImagePayload {
    pub async fn from_url(url: &str) -> anyhow::Result<Self> {
        let client = Client::new();
        let response = client.get(url).send().await?;
        let bytes = response.bytes().await?;
        let image = Image::<Rgba>::from_bytes_inferred(bytes.to_vec().as_slice())?;
        let format = image.format();

        Ok(ImagePayload { image, format })
    }
}

impl FromRequest for ImagePayload {
    type Error = ErrorResponse;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let value = req.extensions().get::<ImagePayload>().cloned();

        let result = match value {
            Some(v) => Ok(v),
            None => Err(ErrorResponse {
                message: "idk no image found".to_owned(),
            }),
        };

        ready(result)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub message: String,
}

impl std::fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Empty response")
    }
}

impl From<anyhow::Error> for ErrorResponse {
    fn from(e: anyhow::Error) -> Self {
        println!("anyhow: {:#?}", e);
        ErrorResponse {
            message: e.to_string(),
        }
    }
}

impl From<ril::Error> for ErrorResponse {
    fn from(e: ril::Error) -> Self {
        println!("ril: {:#?}", e);
        ErrorResponse {
            message: e.to_string(),
        }
    }
}

impl error::ResponseError for ErrorResponse {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::OK
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::InternalServerError()
            .insert_header(CacheControl(vec![CacheDirective::NoCache]))
            .json(self)
    }
}

pub struct ImageResponse {
    pub data: Arc<Mutex<Image<Rgba>>>,
    pub format: ImageFormat,
}

impl ImageResponse {
    pub fn new(image: Image<Rgba>, format: ImageFormat) -> Self {
        Self {
            data: Arc::new(Mutex::new(image)),
            format,
        }
    }

    async fn encode_image(self: Arc<Self>) -> Result<Vec<u8>, ril::Error> {
        // Clone self to move into the closure
        let self_clone = self.clone();
        let encoded_bytes: Result<Result<Vec<u8>, ()>, JoinError> =
            tokio::task::spawn_blocking(move || {
                let data = self_clone.data.lock().unwrap();
                let mut bytes = Vec::new();
                let _ = data.encode(self_clone.format, &mut bytes);
                Ok(bytes)
            })
            .await;

        Ok(encoded_bytes.unwrap().unwrap())
    }

    // fn etag_value(bytes: &[u8]) -> String {
    //     format!("{:x}", md5::compute(bytes))
    // }

    fn content_type(&self) -> String {
        match self.format {
            ImageFormat::Png => ContentType::png().to_string(),
            ImageFormat::Jpeg => ContentType::jpeg().to_string(),
            ImageFormat::WebP => "image/webp".to_string(),
            ImageFormat::Gif => "image/gif".to_string(),
            _ => ContentType::octet_stream().to_string(),
        }
    }
}

#[async_trait::async_trait]
pub trait IntoHttpResponse {
    async fn into_http_response(self) -> Result<HttpResponse, ErrorResponse>;
}

#[async_trait::async_trait]
impl IntoHttpResponse for Arc<ImageResponse> {
    async fn into_http_response(self) -> Result<HttpResponse, ErrorResponse> {
        let bytes = self.clone().encode_image().await?;
        // let etag_value = ImageResponse::etag_value(&bytes);

        Ok(HttpResponse::Ok()
            .content_type(self.content_type())
            // .insert_header(CacheControl(vec![CacheDirective::MaxAge(360u32)]))
            // .insert_header(ETag(EntityTag::new_strong(etag_value.clone())))
            // .insert_header(IfNoneMatch::Items(vec![EntityTag::new(
            //     false,
            //     etag_value,
            // )]))
            .body(bytes))
    }
}
