use axum::{extract::Path, response::IntoResponse};

use crate::{connect_database, error::Error};

pub async fn get(Path(page): Path<String>) -> Result<impl IntoResponse, Error> {
    let db_url = std::env::var("DATABASE_URL")?;
    let pool = connect_database(&db_url).await?;

    match sqlx::query!(r#"SELECT page_id, page_title FROM pages"#)
        .fetch_one(&pool)
        .await
    {
        Ok(record) => Ok(format!("Found page: '{}'", record.page_title)),
        Err(e) => match e {
            sqlx::Error::RowNotFound => page_not_found(&page).await,
            e => Err(e.into()),
        },
    }
}

pub async fn page_not_found(page: &str) -> Result<String, Error> {
    Ok(format!("Could not find '{page}'"))
}
