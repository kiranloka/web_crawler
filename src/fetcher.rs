use crate::frontier::CrawlTask;
use anyhow::Result;
use reqwest::Client;
use tokio::time::{sleep, Duration};

#[derive(Clone)]
pub struct Fetcher {
    client: Client,
    delay_ms: u64,
}

impl Fetcher {
    pub fn new(delay_ms: u64) -> Self {
        let client = Client::builder()
            .user_agent("RustCrawler/0.1")
            .build()
            .unwrap();

        Self { client, delay_ms }
    }

    pub async fn fetch(&self, task: &CrawlTask) -> Result<String> {
        sleep(Duration::from_millis(self.delay_ms)).await;

        let resp = self.client.get(task.url.as_str()).send().await?;
        let body = resp.text().await?;
        Ok(body)
    }
}
