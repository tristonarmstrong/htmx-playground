use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::{errors::ApiError, templates};

pub async fn portfolio() -> impl IntoResponse {
    templates::Portfolio
}

pub async fn notes() -> impl IntoResponse {
    templates::Notes
}

pub async fn tuts() -> impl IntoResponse {
    templates::Tuts
}

pub async fn styles() -> Result<impl IntoResponse, ApiError> {
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/css")
        .body(include_str!("../templates/styles.css").to_owned())?;

    Ok(response)
}
