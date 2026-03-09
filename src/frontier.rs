use dashmap::DashSet;
use url::Url;

#[derive(Debug, Clone)]
pub struct CrawlTask {
    pub url: Url,
    pub depth: u8,
}

#[derive(Clone)]
pub struct Frontier {
    visited: DashSet<String>,
    max_depth: u8,
}

impl Frontier {
    pub fn new(max_depth: u8) -> Self {
        Self {
            visited: DashSet::new(),
            max_depth,
        }
    }

    pub fn seed(&self, seeds: Vec<Url>) -> Vec<CrawlTask> {
        seeds
            .into_iter()
            .map(|url| CrawlTask { url, depth: 0 })
            .collect()
    }

    pub fn should_enqueue(&self, task: &CrawlTask) -> bool {
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
