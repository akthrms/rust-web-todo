use super::ValidatedJson;
use crate::repositories::todo::{CreateTodo, TodoRepository, UpdateTodo};
use axum::{
    extract::{Extension, Path},
    response::IntoResponse,
    Json,
};
use hyper::StatusCode;
use std::sync::Arc;

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
