use axum::{extract::Path, response::Response};
use handlebars::Handlebars;
use serde_json::json;

use crate::{connect_database, error::Error};

pub async fn get(Path(page): Path<String>) -> Result<Response, Error> {
    let db_url = std::env::var("DATABASE_URL")?;
    let pool = connect_database(&db_url).await?;

    match sqlx::query!(r#"SELECT page_id, page_title FROM pages"#)
        .fetch_one(&pool)
        .await
    {
        Ok(record) => Ok(Response::new(
            format!("Found page: '{}'", record.page_title).into(),
        )),
        Err(e) => match e {
            sqlx::Error::RowNotFound => page_not_found(&page).await,
            e => Err(e.into()),
        },
    }
}

pub async fn page_not_found(_page: &str) -> Result<Response, Error> {
    let handlebars = Handlebars::new();
    Response::builder()
        .header("Content-Type", "text/html")
        .body(
            handlebars
                .render_template(
                    include_str!("../../assets/templates/not-found.handlebars"),
                    &json!("{}"),
                )
                .expect("Handlebar threw an error!")
                .into(),
        )
        .map_err(|e| e.into())
}
