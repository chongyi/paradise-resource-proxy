use nipper::Document;
use regex::Regex;
use reqwest::{Client, Url};
use lazy_static::lazy_static;

use crate::{Error, Result};

#[derive(Debug, Serialize)]
pub struct CategoryItem {
    name: String,
    link: String,
}

#[derive(Debug, Serialize)]
pub struct BasePostItem {
    path: String,
    title: String,
}

#[derive(Debug, Serialize)]
pub struct Post {
    title: String,
    post: String,
    images: Vec<String>,
    comments: Vec<PostComment>,
}

#[derive(Debug, Serialize)]
pub struct PostComment {
    comment: String,
}

#[async_trait]
pub trait Catcher {
    async fn get_categories(&self) -> Result<Vec<CategoryItem>>;
    async fn get_category_last_page(&self, cat_path: String) -> Result<u64>;
    async fn get_base_posts(&self, cat_path: String, page: u64) -> Result<Vec<BasePostItem>>;
    async fn get_post(&self, post_path: String) -> Result<Post>;
}

lazy_static! {
    static ref GET_PAGE: Regex = Regex::new("\\d+").unwrap();
}


#[async_trait]
impl Catcher for Client {
    async fn get_categories(&self) -> Result<Vec<CategoryItem>> {
        let html = self
            .get("http://bhc339.top/archiver/")
            .send()
            .await?
            .text()
            .await?;

        {
            let document = Document::from(&html);

            let list_container = document.select("#content > ul:nth-child(2) > li");

            let mut list = vec![];
            list_container.iter().for_each(|item| {
                let link = item.select("a");

                list.push(CategoryItem {
                    name: link.text().to_string(),
                    link: link.attr_or("href", "").to_string(),
                });
            });

            Ok(list)
        }
    }

    async fn get_category_last_page(&self, cat_path: String) -> Result<u64> {
        let html = self
            .get(&format!(
                "http://bhc339.top/archiver/{}",
                cat_path
            ))
            .send()
            .await?
            .text()
            .await?;

        let full_cat_path = {
            let document = Document::from(&html);

            document.select("#end > a").attr("href").ok_or(Error::CustomStrError("找不到地址"))?.to_string()
        };

        let url = Url::parse("http://bhc339.top/archiver").unwrap();
        let url = url.join(&full_cat_path).or(Err(Error::CustomStrError("设置文章分类页地址错误")))?;
        
        let html = self.get(url)
            .send()
            .await?
            .text()
            .await?;

        let document =Document::from(&html);
        let last_page = document.select("#fd_page_bottom > div > a.last").text().to_string();

        let page = GET_PAGE.captures(&last_page).map(|caps| caps[0].parse::<u64>().unwrap_or(1))
            .unwrap_or(1);
        Ok(page)
    }

    async fn get_base_posts(&self, cat_path: String, page: u64) -> Result<Vec<BasePostItem>> {
        let html = self
            .get(&format!(
                "http://bhc339.top/archiver/{}?page={}",
                cat_path, page
            ))
            .send()
            .await?
            .text()
            .await?;

        let document = Document::from(&html);

        let list = document
            .select("#content li > a")
            .iter()
            .map(|item| BasePostItem {
                path: item.attr("href").unwrap().to_string(),
                title: item.text().to_string(),
            })
            .collect();

        Ok(list)
    }

    async fn get_post(&self, post_path: String) -> Result<Post> {
        let html = self
            .get(&format!("http://bhc339.top/{}", post_path))
            .send()
            .await?
            .text()
            .await?;

        let document = Document::from(&html);

        let title = document.select("#thread_subject").text().to_string();
        let post = document.select(".t_f");
        let images = document
            .select(".pattl img.zoom")
            .iter()
            .filter_map(|item| item.attr("file").map(|v| v.to_string()))
            .collect();

        let mut post_iter = post.iter();

        let main_post = post_iter
            .next()
            .ok_or(Error::CustomStrError("未找到帖子内容"))?;
        let comments = post_iter
            .map(|item| PostComment {
                comment: item.text().to_string(),
            })
            .collect();

        Ok(Post {
            title,
            post: main_post.html().to_string(),
            images,
            comments,
        })
    }
}
