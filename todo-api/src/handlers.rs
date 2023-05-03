use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    BoxError, Json,
};
use hyper::StatusCode;
use serde::de::DeserializeOwned;
use validator::Validate;

pub mod label;
pub mod todo;

pub struct ValidatedJson<T>(T);

#[async_trait]
impl<T, B> FromRequest<B> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    B: http_body::Body + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req).await.map_err(|rejection| {
            (
                StatusCode::BAD_REQUEST,
                format!("Json parse error: [{}]", rejection),
            )
        })?;

        value.validate().map_err(|rejection| {
            (
                StatusCode::BAD_REQUEST,
                format!("Validation error: [{}]", rejection).replace('\n', ", "),
            )
        })?;

        Ok(ValidatedJson(value))
    }
}
