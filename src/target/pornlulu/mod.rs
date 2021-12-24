use axum::{
    extract::{Path, Query, TypedHeader},
    routing::get,
    Json, Router,
};
use headers::UserAgent;
use reqwest::Client;
use serde_json::{json, Value};

pub mod category;
pub mod list;
pub mod post;

pub fn routes() -> Router {
    Router::new()
        .route("/category", get(get_category_list))
        .route("/category/:category/post", get(get_post_list))
        .route("/post/:post", get(get_post))
}

async fn get_category_list(TypedHeader(user_agent): TypedHeader<UserAgent>) -> Json<Vec<Value>> {
    let client = Client::builder().user_agent(user_agent.as_str()).build();

    let result = match client {
        Ok(client) => {
            category::get_category_list(client, "https://www.pornlulu.com/zh-hans/category")
                .await
                .unwrap_or(vec![])
        }
        Err(_) => vec![],
    };

    Json(
        result
            .into_iter()
            .map(|item| {
                json!({
                    "id": item.category_id,
                    "title": item.name,
                })
            })
            .collect(),
    )
}

#[derive(Deserialize)]
pub struct Paginate {
    page: u32,
}

async fn get_post_list(
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    Path((category_id,)): Path<(u32,)>,
    Query(Paginate { page }): Query<Paginate>,
) -> Json<Value> {
    let client = Client::builder().user_agent(user_agent.as_str()).build();

    let result = match client {
        Ok(client) => list::get_list(
            client,
            format!(
                "https://www.pornlulu.com/zh-hans/cat/{}?page={}",
                category_id, page
            )
            .as_str(),
        )
        .await
        .unwrap_or(None),
        Err(_) => None,
    };

    match result {
        Some(list) => Json(json!({
            "has_more": list.current != list.last,
            "data": list.items.into_iter().map(|item| json!({
                "id": item.id,
                "title": item.title,
                "thumbnail": item.cover,
            })).collect::<Vec<Value>>()
        })),
        None => Json(json!({"has_more": false, "data": Vec::<()>::new()})),
    }
}

async fn get_post(
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    Path((content_id,)): Path<(String,)>,
) -> Json<Value> {
    let client = Client::builder().user_agent(user_agent.as_str()).build();

    let result = match client {
        Ok(client) => post::get_post(
            client,
            format!("https://www.pornlulu.com/zh-hans/v/{}", content_id).as_str(),
        )
        .await
        .unwrap_or(None),
        Err(_) => None,
    };

    match result {
        Some(post) => Json(json!({
            "title": post.title,
            "keywords": post.keywords,
            "description": post.description,
            "content": serde_json::to_string(&json!({
                "source": post.source
            })).unwrap()
        })),
        None => Json(Value::Null),
    }
}
