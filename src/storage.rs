use crate::frontier::CrawlTask;

pub async fn store_page(task: &CrawlTask, title: &Option<String>) {
    println!(
        "STORED page:{} depth={} title={}",
        task.url,
        task.depth,
        title.as_deref().unwrap_or("N/A")
    )
}
