use std::sync::Arc;

use axum::{Router, AddExtensionLayer, routing::get};
use reqwest::{Client, cookie};

use self::common::login;

pub mod category;
pub mod common;

pub fn routes() -> Router {
    let client = Client::builder()
        .cookie_store(true)
        .cookie_provider(Arc::new(cookie::Jar::default()))
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4692.71 Safari/537.36")
        .build()
        .unwrap();

    Router::new()
        .route("/login", get(login))
        .layer(AddExtensionLayer::new(client))
}