use nipper::Document;
use regex::Regex;
use reqwest::Client;
use lazy_static::lazy_static;
use anyhow::Result;

lazy_static! {
    static ref GET_SOURCE: Regex = Regex::new("src:'(.*?)'").unwrap();
    static ref GET_SCRIPT: Regex = Regex::new("eval\\((.*)\\)").unwrap();
}

#[derive(Debug, Clone)]
pub struct PostTagRawItem {
    pub name: String,
    pub href: String,
}


#[derive(Debug, Clone)]
pub struct PostRaw {
    pub title: String,
    pub keywords: String,
    pub description: String,
    pub source: String,
    pub tags: Vec<PostTagRawItem>,
}

pub async fn get_post(client: Client, path: &str) -> Result<Option<PostRaw>> {
    let html = client.get(path).send().await?.text().await?;
    let document = Document::from(&html);

    // 取标签
    let tags = document.select(".container-fluid > .d-none > a");
    let tags: Vec<PostTagRawItem> = if tags.exists() {
        tags.iter()
            .map(|item| {
                item.attr("href")
                    .and_then(|href| {
                        if item.text().is_empty() {
                            None
                        } else {
                            Some(PostTagRawItem {
                                name: item.text().to_string(),
                                href: href.to_string(),
                            })
                        }
                    })
            })
            .filter(|s| s.is_some())
            .map(|s| s.unwrap())
            .collect()
    } else {
        vec![]
    };

    let title = document.select("body > div.wrapper > div.content-wrapper > div > div > h1 > a").text().trim().to_string();
    let keywords = tags.iter()
        .map(|item| item.name.as_str()).collect::<Vec<&str>>().join(",")
        .to_string();
    let description = document.select("meta[name=description]").attr_or("content", "").to_string();

    // 该页面有个坑爹的地方就是实际视频列表数据文件在脚本里，
    // 且这个脚本被混淆过，所以需要动用脚本代码分析器进行解析，
    // 这里使用基于 deno 的 js 脚本执行器。但是首先需要取得代码段。
    let script = document.select(".container-fluid > div.fullwidth > script").last();

    if script.exists() {
        let script = script.text().to_string();

        // 取得混淆代码
        let confused_script = GET_SCRIPT.captures(&script)
            .ok_or(anyhow::Error::msg("Captures (full script -> confuse script) failed."))?;

        // 代码解析
        let actual_script = js_sandbox::eval_json(&format!("({})", confused_script[1].to_string()))?;

        // 取出代码中的真实视频地址
        let source = GET_SOURCE.captures(actual_script.as_str().unwrap())
            .ok_or(anyhow::Error::msg("Captures (actual script -> source) failed."))?;

        Ok(Some(PostRaw {
            title,
            keywords,
            description,
            source: source[1].to_string(),
            tags,
        }))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use reqwest::{Client, Proxy};

    use crate::target::pornlulu::post::get_post;

    #[tokio::test]
    async fn test_get_post() {
        let client = Client::builder().proxy(Proxy::all("socks5://127.0.0.1:7890").unwrap()).build().unwrap();
        let result = get_post(client, "https://www.pornlulu.com/v/nww8o").await.unwrap();

        println!("{:?}", result);
        assert!(result.is_some())
    }
}