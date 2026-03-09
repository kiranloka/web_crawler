use dashmap::DashSet;
use futures::stream::{FuturesUnordered, StreamExt};
use std::collection::VecDeque;
use tokio::sync::mpsc;
use url::Url;

#[derive(Debug, Clone)]
pub struct CrawlTalk {
    pub url: Url,
    pub depth: u8,
}

#[derive(Clone)]
pub struct Frontier {
    visited: Dashset,
    max_depth: u8,
}

impl Frontier {
    pub fn new(max_depth: u8) -> Self {
        Self {
            visited: DashSet::new(),
            max_depth,
        }
    }

    pub fn seed(&self, seeds: Vec<Url>) -> Vec<CrawlTalk> {
        seeds
            .into_iter()
            .map(|url| CrawlTalk { url, depth: 0 })
            .collect()
    }

    pub fn should_enqueue(&self, task: &CrawlTalk) -> bool {
        if task.depth > self.max_depth {
            return false;
        }

        let key = task.url.to_string();
        if self.visited.contains(&key) {
            false
        } else {
            self.visited.insert(key);
            true
        }
    }
}
