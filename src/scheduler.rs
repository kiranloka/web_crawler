use crate::frontier::{CrawlTask, Frontier};
use tokio::sync::mpsc;

#[derive(Clone)]
pub struct Scheduler {
    frontier: Frontier,
    tx: mpsc::Sender<CrawlTask>,
}

impl Scheduler {
    pub fn new(frontier: Frontier, tx: mpsc::Sender<CrawlTask>) -> Self {
        Self { frontier, tx }
    }

    pub async fn submit(&self, task: CrawlTask) {
        if self.frontier.should_enqueue(&task) {
            let _ = self.tx.send(task).await;
        }
    }
}
