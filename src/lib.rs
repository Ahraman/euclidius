use sqlx::{migrate::MigrateDatabase, postgres::PgPoolOptions, PgPool};

use crate::error::Error;

pub mod error;
pub mod route;

pub async fn validate_database() -> Result<(), Error> {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");

    if !sqlx::Postgres::database_exists(&url).await? {
        sqlx::Postgres::create_database(&url).await?;
    }

    let conn = connect_database(&url).await?;

    let migrator = sqlx::migrate!();
    migrator.run(&conn).await?;

    Ok(())
}

pub async fn connect_database(url: &str) -> Result<PgPool, Error> {
    PgPoolOptions::new()
        .connect(url)
        .await
        .map_err(|e| e.into())
}
