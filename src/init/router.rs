use axum::{routing::{get, post}, Router};
use crate::controller;

pub fn init_router() -> Router {
    let app = Router::new()
        .merge(user_router());
    return app;
}

fn user_router() -> Router {
    let app = Router::new()
        .route("/users", get(controller::users))
        .route("/users", post(controller::create_user));
    return app;
}