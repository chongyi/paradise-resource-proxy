use crate::{Result, Error};
use axum::extract::Extension;
use axum_debug::debug_handler;
use nipper::Document;
use reqwest::{Client, Url};

#[debug_handler]
pub async fn login(Extension(client): Extension<Client>) -> Result<String> {
    let html = client
        .get("http://bhc339.top/member.php?mod=logging&action=login")
        .send()
        .await?
        .text()
        .await?;

    let (action, form_hash_val, referer_val) = {
        let document = Document::from(&html);

        let form = document.select("form[name=login]");
        let form_hash = form.select("input[name=formhash]");
        let referer = form.select("input[name=referer]");
    
        let action = form.attr("action").ok_or(Error::CustomStrError("[baihua::login] Cannot get form action"))?.to_string();
        let form_hash_val = form_hash.attr("value").ok_or(Error::CustomStrError("[baihua::login] Cannot get form hash value"))?.to_string();
        let referer_val = referer.attr("value").ok_or(Error::CustomStrError("[baihua::login] Cannot get referer"))?.to_string();

        (action, form_hash_val, referer_val)
    };

    log::debug!("Baihua form -> action = {}", action);
    log::debug!("Baihua form -> formhash = {}", form_hash_val);
    log::debug!("Baihua form -> referer = {}", referer_val);

    let form_data = [
        ("formhash", form_hash_val.as_str()),
        ("referer", referer_val.as_str()),
        ("loginfield", "username"),
        ("username", "osexoff1"),
        ("password", "osexoff1"),
        ("questionid", "0"),
        ("answer", "")
    ];

    let url = Url::parse(&format!("http://bhc339.top/{}", action))?;

    let res = client.clone().post(url)
        .form(&form_data)
        .send()
        .await?
        .text()
        .await?;

    Ok(res)
}
