pub mod dev_db;

use crate::ctx::Ctx;
use tokio::sync::OnceCell;
use tracing::info;

use crate::model;
use crate::model::task::{Task, TaskBmc, TaskForCreate};

use crate::model::ModelManager;

pub async fn init_dev() {
    static INIT: OnceCell<()> = OnceCell::const_new();
    INIT.get_or_init(|| async {
        info!("{:<12} - init_dev_all()", "FOR-DEV-ONLY");
        if let Err(e) = dev_db::init_dev_db().await {
            info!("Error initializing dev db: {:?}", e);
        }
    })
    .await;
}

pub async fn seed_tasks(ctx: &Ctx, mm: &ModelManager, titles: &[&str]) -> model::Result<Vec<Task>> {
    let mut tasks = Vec::new();
    for title in titles {
        let id = TaskBmc::create(
            ctx,
            mm,
            TaskForCreate {
                title: title.to_string(),
            },
        )
        .await?;
        let task = TaskBmc::get(ctx, mm, id).await?;
        tasks.push(task);
    }
    Ok(tasks)
}

pub async fn init_test() -> ModelManager {
    static INIT: OnceCell<ModelManager> = OnceCell::const_new();

    let mm = INIT
        .get_or_init(|| async {
            init_dev().await;
            ModelManager::new().await.unwrap()
        })
        .await;
    mm.clone()
}
