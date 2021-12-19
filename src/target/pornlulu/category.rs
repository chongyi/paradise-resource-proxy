use nipper::Document;
use regex::Regex;
use reqwest::Client;
use lazy_static::lazy_static;
use anyhow::Result;

#[derive(Clone, Debug)]
pub struct CategoryRawItem {
    pub category_id: u32,
    pub name: String,
    pub href: String,
}

lazy_static! {
    static ref GET_ID: Regex = Regex::new("\\d+").unwrap();
}

pub async fn get_category_list(client: Client, path: &str) -> Result<Vec<CategoryRawItem>> {
    let html = client.get(path).send().await?.text().await?;

    let document = Document::from(&html);

    // 通过选择器找到列表
    let source_container = document.select(".content-wrapper > .content > .container-fluid > .row > div");

    // 判断数据长度
    let category_items = if source_container.exists() {
        source_container.iter()
            .map(|item| {
                let a = item.select("a");

                if !a.exists() || a.attr("href").is_none() || a.text().is_empty() {
                    None
                } else {
                    let href = a.attr("href").map(|s| s.to_string()).unwrap();
                    let name = a.text().to_string();

                    Some((name, href))
                }
            })
            .filter(|res| res.is_some())
            .map(|res|
                res.map(|(name, href)| {
                    let category_id = GET_ID.captures(&href).map(|caps| caps[0].parse::<u32>().unwrap_or(0))
                        .unwrap_or(0);

                    log::info!("Parsed category, id = {}, name = {}", category_id, name.as_str());
                    CategoryRawItem {
                        category_id,
                        name,
                        href,
                    }
                }).unwrap()
            )
            .collect()
    } else {
        vec![]
    };

    Ok(category_items)
}