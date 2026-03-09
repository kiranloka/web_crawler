use crate::frontier::CrawlTask;
use scraper::{Html, Selector};
use url::Url;

pub struct ParseResult {
    pub new_links: Vec<CrawlTask>,
    pub title: Option<String>,
}

pub fn parse_page(base: &Url, depth: u8, html: &str) -> ParseResult {
    let document = Html::parse_document(html);

    let title_sel = Selector::parse("title").unwrap();
    let title = document
        .select(&title_sel)
        .next()
        .and_then(|t| Some(t.text().collect::<String>().trim().to_string()))
        .filter(|s: &String| !s.is_empty());

    let link_sel = Selector::parse("a[href]").unwrap();

    let mut new_links = Vec::new();

    for el in document.select(&link_sel) {
        if let Some(href) = el.value().attr("href") {
            if let Ok(next_url) = base.join(href) {
                if matches!(next_url.scheme(), "http" | "https") {
                    new_links.push(CrawlTask {
                        url: next_url,
                        depth: depth + 1,
                    });
                }
            }
        }
    }

    ParseResult { new_links, title }
}
