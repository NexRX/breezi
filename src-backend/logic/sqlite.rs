use sqlx::{
    SqlitePool,
    migrate::{MigrateDatabase as _, Migrator},
};

use crate::model::Config;

// Embed migrations at compile time
static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

/// Creates DB -> Run Migrations -> Creates Connect Pool
pub async fn setup_database(config: &Config) -> crate::Result<SqlitePool> {
    let db_url = config.database.to_string_lossy();

    if !sqlx::Sqlite::database_exists(&db_url).await? {
        sqlx::Sqlite::create_database(&db_url).await?;
    }

    // Connect to the database
    let db = SqlitePool::connect(&db_url).await?;

    // Run embedded migrations
    MIGRATOR.run(&db).await?;

    Ok(db)
}
