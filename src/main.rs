use axum::Router;

mod target;

#[tokio::main]
async fn main() {
    let router = Router::new().nest("/pornlulu", target::pornlulu::routes());

    axum::Server::bind(&"0.0.0.0:8010".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
