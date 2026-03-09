mod fetcher;
mod frontier;
mod parser;
mod scheduler;
mod storage;

use tokio::sync::mpsc;
use tokio::task;
use url::Url;

use crate::{
    fetcher::Fetcher, frontier::Frontier, parser::parse_page, scheduler::Scheduler,
    storage::store_page,
};
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let max_depth = 2;
    let concurrency: usize = 8;
    let delay_ms = 100;

    let frontier = Frontier::new(max_depth);
    let (tx, rx) = mpsc::channel(1024);
    let scheduler = Scheduler::new(frontier.clone(), tx.clone());
    let fetcher = Fetcher::new(delay_ms);

    println!(
        r#"
  ░██████              ░██       ░██                                                                         ░██ 
 ░██   ░██                       ░██                                                                         ░██ 
░██         ░████████  ░██ ░████████  ░███████  ░██░████     ░███████  ░██░████  ░██████   ░██    ░██    ░██ ░██ 
 ░████████  ░██    ░██ ░██░██    ░██ ░██    ░██ ░███        ░██    ░██ ░███           ░██  ░██    ░██    ░██ ░██ 
        ░██ ░██    ░██ ░██░██    ░██ ░█████████ ░██         ░██        ░██       ░███████   ░██  ░████  ░██  ░██ 
 ░██   ░██  ░███   ░██ ░██░██   ░███ ░██        ░██         ░██    ░██ ░██      ░██   ░██    ░██░██ ░██░██   ░██ 
  ░██████   ░██░█████  ░██ ░█████░██  ░███████  ░██          ░███████  ░██       ░█████░██    ░███   ░███    ░██ 
            ░██                                                                                                  
            ░██                                                                                                  
                                                                                                                 
"#
    );

    use std::io::{self, Write};
    print!("Enter the URL to crawl: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();

    if input.is_empty() {
        eprintln!("Error: URL cannot be empty.");
        return Ok(());
    }

    let start_url = Url::parse(input)?;

    let seeds = vec![start_url];
    for task in frontier.seed(seeds) {
        scheduler.submit(task).await;
    }

    let rx = Arc::new(Mutex::new(rx));
    let mut workers = Vec::new();
    for _ in 0..concurrency {
        let rx = Arc::clone(&rx);
        let scheduler = scheduler.clone();
        let fetcher = fetcher.clone();
        let handle = task::spawn(async move {
            loop {
                let task = {
                    let mut rx_lock = rx.lock().await;
                    rx_lock.recv().await
                };

                let task = match task {
                    Some(t) => t,
                    None => break,
                };

                println!("Crawling: {}", task.url);

                let body = match fetcher.fetch(&task).await {
                    Ok(b) => b,
                    Err(e) => {
                        eprintln!("Failed to fetch {}: {}", task.url, e);
                        continue;
                    }
                };

                let parsed = parse_page(&task.url, task.depth, &body);
                if let Some(title) = &parsed.title {
                    println!("Title: {}", title);
                }

                store_page(&task, &parsed.title).await;

                for new_task in parsed.new_links {
                    scheduler.submit(new_task).await;
                }
            }
        });

        workers.push(handle);
    }

    drop(tx);
    for w in workers {
        let _ = w.await;
    }

    Ok(())
}
