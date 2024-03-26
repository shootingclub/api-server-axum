use axum::http::StatusCode;
use axum::Json;
use crate::entity::{CreateUser, User};
use crate::comm::{SuccessResp};
use crate::service;
use crate::service::create_users;

pub async fn users() -> Json<SuccessResp<Vec<User>>> {
    return Json(SuccessResp {
        status: 200,
        message: String::from("OK"),
        data: service::get_users(),
    });
}

pub async fn create_user(
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    (StatusCode::CREATED, Json(create_users(payload)))
}