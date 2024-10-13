use axum::{routing::post, Router};
use registry::AppRegistryImpl;

use crate::handler::auth::{login, logout};

pub fn routes() -> Router<AppRegistryImpl> {
    let auth_router = Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout));

    Router::new().nest("/auth", auth_router)
}
