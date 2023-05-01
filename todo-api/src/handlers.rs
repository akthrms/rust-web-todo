use crate::repositories::{CreateTodo, TodoRepository, UpdateTodo};
use axum::{
    async_trait,
    extract::{Extension, FromRequest, Path, RequestParts},
    response::IntoResponse,
    BoxError, Json,
};
use hyper::StatusCode;
use serde::de::DeserializeOwned;
use std::sync::Arc;
use validator::Validate;

pub async fn create_todo<T: TodoRepository>(
    ValidatedJson(payload): ValidatedJson<CreateTodo>,
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok((
        StatusCode::CREATED,
        Json(
            repository
                .create(payload)
                .await
                .or(Err(StatusCode::NOT_FOUND))?,
        ),
    ))
}

pub async fn find_todo<T: TodoRepository>(
    Path(id): Path<i32>,
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok((
        StatusCode::OK,
        Json(repository.find(id).await.or(Err(StatusCode::NOT_FOUND))?),
    ))
}

pub async fn all_todo<T: TodoRepository>(
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok((StatusCode::OK, Json(repository.all().await.unwrap())))
}

pub async fn update_todo<T: TodoRepository>(
    Path(id): Path<i32>,
    ValidatedJson(payload): ValidatedJson<UpdateTodo>,
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok((
        StatusCode::CREATED,
        Json(
            repository
                .update(id, payload)
                .await
                .or(Err(StatusCode::NOT_FOUND))?,
        ),
    ))
}

pub async fn delete_todo<T: TodoRepository>(
    Path(id): Path<i32>,
    Extension(repository): Extension<Arc<T>>,
) -> StatusCode {
    repository
        .delete(id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .unwrap_or(StatusCode::NOT_FOUND)
}

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
