use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use tracing::info;

type Db = Pool<Postgres>;

const PG_DEV_POSTGRES_URL: &str = "postgres://postgres:postgres@localhost:5432/db";
const PG_DEV_APP_URL: &str = "postgres://postgres:postgres@localhost/db";

//sql files
const SQL_RECREATE_DB: &str = "sql/dev_initial/00-recreate-db.sql";
const SQL_DIR: &str = "sql/dev_initial/";

pub async fn init_dev_db() -> Result<(), Box<dyn std::error::Error>> {
    info!("{:<12} - init_dev_db()", "FOR-DEV-ONlY");
    {
        pexec(&new_db_pool(PG_DEV_POSTGRES_URL).await?, SQL_RECREATE_DB).await?;
    }

    let mut paths: Vec<PathBuf> = fs::read_dir(SQL_DIR)?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .collect();
    paths.sort();

    let app_db = new_db_pool(PG_DEV_APP_URL).await?;

    for path in paths {
        if let Some(path) = path.to_str() {
            let path = path.replace('\\', "/");
            if path.ends_with(".sql") && path != SQL_RECREATE_DB {
                pexec(&app_db, &path).await?;
            }
        }
    }
    Ok(())
}

async fn pexec(db: &Db, file: &str) -> Result<(), sqlx::Error> {
    info!("{:<12} - pexec: {file}", "FOR-DEV-ONLY");

    let content = fs::read_to_string(file)?;
    info!("{:<12} - pexec: {content}", "FOR-DEV-ONLY");
    let sqls: Vec<&str> = content.split(";").collect();
    info!("{:<12} - pexec: {:?} sqls", "FOR-DEV-ONLY", sqls);

    for sql in sqls {
        sqlx::query(sql).execute(db).await?;
        info!("{:<12} - pexec: {sql}", "FOR-DEV-ONLY");
    }
    Ok(())
}

async fn new_db_pool(db_con_url: &str) -> Result<Db, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_millis(500))
        .connect(db_con_url)
        .await
}
