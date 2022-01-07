use crate::Result;
use axum::extract::Extension;
use nipper::Document;
use reqwest::Client;

pub async fn login(Extension(client): Extension<Client>) -> Result<String> {
    let html = client
        .get("http://bhc339.top/member.php?mod=logging&action=login")
        .send()
        .await?
        .text()
        .await?;

    let document = Document::from(&html);

    let form = document.select("form[name=login]");



    Ok("ok".to_string())
}
