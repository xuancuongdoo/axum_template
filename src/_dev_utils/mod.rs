pub mod dev_db;

extern crate dotenv;
use dotenv::dotenv;
use std::env;

use tokio::sync::OnceCell;
use tracing::info;

use crate::ctx::Ctx;
use crate::model;
use crate::model::task::{Task, TaskBmc, TaskForCreate};
use crate::model::ModelManager;

pub async fn init_dev() {
    static INIT: OnceCell<()> = OnceCell::const_new();
    dotenv().ok();
    match env::var("SERVICE_DB_URL") {
        Ok(url) => {
            println!("SERVICE_DB_URL={url}");
        }
        Err(_) => println!("SERVICE_DB_URL=none"),
    }

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
    dotenv().ok();
    match env::var("SERVICE_DB_URL") {
        Ok(url) => {
            println!("SERVICE_DB_URL={url}");
        }
        Err(_) => println!("SERVICE_DB_URL=none"),
    }

    let mm = INIT
        .get_or_init(|| async {
            init_dev().await;
            ModelManager::new().await.unwrap()
        })
        .await;
    mm.clone()
}
