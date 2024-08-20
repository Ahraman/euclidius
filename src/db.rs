use sqlx::{migrate::MigrateDatabase, postgres::PgPoolOptions, PgPool, Postgres};

use crate::result::Result;

pub async fn connect_or_setup(database_url: &str) -> Result<PgPool> {
    if !Postgres::database_exists(database_url).await? {
        Postgres::create_database(database_url).await?;
    }

    let conn = PgPoolOptions::new().connect(database_url).await?;
    sqlx::migrate!().run(&conn).await?;

    Ok(conn)
}
