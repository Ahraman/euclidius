use sqlx::{migrate::MigrateDatabase, postgres::PgPoolOptions, PgPool, Postgres};

pub mod app;
pub mod error;
pub mod request;

use crate::error::Error;

pub async fn validate_database_and_connect() -> Result<PgPool, Error> {
    let url = std::env::var("DATABASE_URL")?;

    if !Postgres::database_exists(&url).await? {
        Postgres::create_database(&url).await?;
    }

    let conn = connect_database(&url).await?;

    let migrator = sqlx::migrate!();
    migrator.run(&conn).await?;

    Ok(conn)
}

pub async fn connect_database(url: &str) -> Result<PgPool, Error> {
    PgPoolOptions::new()
        .connect(url)
        .await
        .map_err(|e| e.into())
}
