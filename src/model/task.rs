use crate::ctx::Ctx;
use crate::model::error::Error;
use crate::model::ModelManager;
use crate::model::Result;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Task {
    pub id: i64,
    pub title: String,
}

#[derive(Deserialize)]
pub struct TaskForCreate {
    pub title: String,
}

#[derive(Deserialize)]
pub struct TaskForUpdate {
    pub title: Option<String>,
}

pub struct TaskBmc;

impl TaskBmc {
    pub async fn create(_ctx: &Ctx, mm: &ModelManager, task_c: TaskForCreate) -> Result<i64> {
        let db = mm.db();
        let (id,) =
            sqlx::query_as::<_, (i64,)>("INSERT INTO task (title) values ($1) returning id")
                .bind(task_c.title)
                .fetch_one(db)
                .await?;
        Ok(id)
    }

    pub async fn list(_ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Task>> {
        let db = mm.db();

        let task: Vec<Task> = sqlx::query_as("SELECT * FROM task ORDER BY id")
            .fetch_all(db)
            .await?;
        Ok(task)
    }
    pub async fn get(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Task> {
        let db = mm.db();
        let task = sqlx::query_as("SELECT * FROM task WHERE id = $1")
            .bind(id)
            .fetch_optional(db)
            .await?
            .ok_or(Error::EntityNotFound { entity: "task", id })?;
        Ok(task)
    }

    pub async fn delete(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<u64> {
        let db = mm.db();
        let count = sqlx::query("DELETE FROM task WHERE id = $1")
            .bind(id)
            .execute(db)
            .await?
            .rows_affected();

        if count == 0 {
            return Err(Error::EntityNotFound { entity: "task", id });
        }

        Ok(count)
    }

    pub async fn update(
        _ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        task_c: TaskForUpdate,
    ) -> Result<()> {
        let db = mm.db();
        let count = sqlx::query("UPDATE task SET title = $1 WHERE id = $2")
            .bind(task_c.title)
            .bind(id)
            .execute(db)
            .await?;
        assert_eq!(count.rows_affected(), 1, "Did not update 1 row");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused)]
    use crate::_dev_utils;

    use super::*;
    use anyhow::Result;
    use serial_test::serial;

    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_title = "test_create_ok title";

        let task_c = TaskForCreate {
            title: fx_title.to_string(),
        };

        let id = TaskBmc::create(&ctx, &mm, task_c).await?;

        let task = TaskBmc::get(&ctx, &mm, id).await?;
        assert_eq!(task.title, fx_title);

        let count = TaskBmc::delete(&ctx, &mm, id).await?;
        assert_eq!(count, 1);

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_get_err_not_found() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_id = 100;

        let res = TaskBmc::get(&ctx, &mm, fx_id).await;

        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "task",
                    id: 100
                })
            ),
            "Not found"
        );

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_list_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_titles = &["test_list_ok title"];
        _dev_utils::seed_tasks(&ctx, &mm, fx_titles).await?;

        let tasks = TaskBmc::list(&ctx, &mm).await?;

        Ok(())
    }
}
