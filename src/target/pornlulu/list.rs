use anyhow::Result;
use nipper::Document;
use reqwest::Client;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref GET_POST_ID: Regex = Regex::new("/v/([0-9a-zA-Z]+)").unwrap();
}


#[derive(Clone, Debug)]
pub struct PostRawItem {
    pub id: String,
    pub title: String,
    pub cover: String,
    pub href: String,
}

#[derive(Clone, Debug)]
pub struct PostRawList {
    pub items: Vec<PostRawItem>,
    pub current: u32,
    pub last: u32,
}


pub async fn get_list(client: Client, path: &str) -> Result<Option<PostRawList>> {
    let html = client.get(path).send().await?.text().await?;

    let document = Document::from(&html);

    let source_container = document.select("#videos > div");
    if source_container.exists() {
        let items = source_container
            .iter()
            .map(|item| {
                let target = item.select(".visited");

                target
                    .first()
                    .attr("href")
                    .map(|s| s.to_string())
                    .and_then(|href| {
                        let res = target
                            .first()
                            .select("img")
                            .attr("data-src")
                            .map(|s| s.to_string());
                        match res {
                            Some(cover) => Some((cover, href)),
                            None => None,
                        }
                    })
                    .and_then(|(cover, href)| {
                        let title = target.last().text();
                        if title.is_empty() {
                            None
                        } else {
                            let result = GET_POST_ID.captures(&href)?;
                            Some(PostRawItem {
                                id: result[1].to_string(),
                                cover,
                                href,
                                title: title.to_string(),
                            })
                        }
                    })
            })
            .filter(|item| item.is_some())
            .map(|s| s.unwrap())
            .collect();

        let current = document
            .select("nav#w0 > ul.pagination > li.active > a")
            .text()
            .to_string()
            .parse::<u32>()
            .unwrap_or(1);

        let last = document
            .select("nav#w0 > ul.pagination > li:nth-last-child(2) > a")
            .text()
            .to_string()
            .parse::<u32>()
            .unwrap_or(1);

        Ok(Some(PostRawList {
            items,
            current,
            last,
        }))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use reqwest::{Client, Proxy};

    use crate::target::pornlulu::list::get_list;

    #[tokio::test]
    async fn test_get_list() {
        let client = Client::builder().proxy(Proxy::all("socks5://127.0.0.1:7890").unwrap()).build().unwrap();
        let result = get_list(client, "https://www.pornlulu.com/cat/119?page=2").await.unwrap();

        assert!(result.is_some())
    }
}