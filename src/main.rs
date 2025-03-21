pub mod async_component;
pub mod components;
pub mod meta;
pub mod models;
pub mod pages;
pub mod render;
pub mod ssg;

use std::{fs, path::Path};

use gray_matter::{engine::YAML, Matter};
use models::article::Article;
use pages::article_page::ArticlePageProps;
use ssg::Ssg;

use crate::pages::{article_page::ArticlePage, home::Homepage};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let articles = list_articles().await?;

    tokio::fs::create_dir_all("./out/articles").await?;

    // initialize the Ssg context
    let ssg = Ssg::new(Path::new("./out"));

    // generate the pages
    ssg.gen("index.html", Homepage).await?;

    for article in articles {
        ssg.gen(&format!("articles/{}.html", article.slug), || {
            ArticlePage(ArticlePageProps { article })
        })
        .await?;
    }

    Ok(())
}

async fn list_articles() -> Result<Vec<Article>, Box<dyn std::error::Error>> {
    let paths = fs::read_dir("./articles")?;

    let mut articles = Vec::with_capacity(10);

    for path in paths {
        let file = path?.path();
        let algo = fs::read_to_string(file.clone())?;
        let matter = Matter::<YAML>::new();
        let Some(parsed_entity) = matter.parse_with_struct(&algo) else {
            continue;
        };
        let content = parsed_entity.content.clone();
        let mut article: Article = parsed_entity.data;
        article.content = content;
        if article.slug.is_empty() {
            // path without extension
            let filename_without_extension = file.file_stem().unwrap().to_str().unwrap();
            article.slug = filename_without_extension.to_string();
        }
        articles.push(article);
    }

    Ok(articles)
}
