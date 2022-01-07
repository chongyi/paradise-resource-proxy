use axum::{
    extract::{Extension, Path, Query},
    Json,
};
use reqwest::Client;

use crate::Result;
use super::catcher::*;

pub async fn get_categories(Extension(client): Extension<Client>) -> Result<Json<Vec<CategoryItem>>> {
    client.get_categories().await.map(Json)
}

pub async fn get_category_last_page(Extension(client): Extension<Client>, Path(path): Path<String>) -> Result<String> {
    Ok(format!("{}", client.get_category_last_page(path).await?))
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Page {
    page: u64
}

impl Default for Page {
    fn default() -> Self {
        Page {
            page: 1
        }
    }
}

pub async fn get_base_posts(Extension(client): Extension<Client>, Path(path): Path<String>, Query(Page { page }): Query<Page>) -> Result<Json<Vec<BasePostItem>>> {    
    client.get_base_posts(path, page).await.map(Json)
}

pub async fn get_post(Extension(client): Extension<Client>, Path(path): Path<String>) -> Result<Json<Post>> {
    client.get_post(path).await.map(Json)
}