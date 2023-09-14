use sqlx::{Pool, Postgres};

type Db = Pool<Postgres>;

const PG_DEV_POSTGRES_URL: &str = "postgres://postgres:postgres@localhost:5432/postgres";
const PG_DEV_APP_URL: &str = "postgres://app_user:1234@localhost/app_db";

//sql files
const SQL_RECREATE_DB: &str = "sql/dev_init/00-recreate-db.sql";
const SQL_DIR: &str = "sql/dev_init";

async fn new_db_pool(db_con_url: &str) -> Result<Db, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_millis(500))
        .connect(db_con_url)
        .await
}
