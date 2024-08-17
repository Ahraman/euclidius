use axum::{
    extract::{Path, Query},
    response::Response,
    Form,
};
use handlebars::Handlebars;
use serde::Deserialize;
use serde_json::json;

use crate::{connect_database, error::Error};

#[derive(Deserialize)]
pub enum Action {
    #[serde(rename = "view")]
    View,
    #[serde(rename = "create")]
    Create,
    #[serde(rename = "submit")]
    Submit,
}

impl Action {
    fn default_action() -> Self {
        Self::View
    }
}

#[derive(Deserialize)]
pub struct PageQuery {
    #[serde(default = "Action::default_action")]
    pub action: Action,
}

#[derive(Deserialize)]
pub struct PageSubmit {
    pub title: String,
}

pub async fn get(
    Path(page): Path<String>,
    Query(query): Query<PageQuery>,
) -> Result<Response, Error> {
    match query.action {
        Action::View | Action::Submit => view_page(&page).await,
        Action::Create => create_page(&page).await,
    }
}

pub async fn post(
    Path(page): Path<String>,
    Query(query): Query<PageQuery>,
    Form(submit): Form<PageSubmit>,
) -> Result<Response, Error> {
    match query.action {
        Action::View => view_page(&page).await,
        Action::Create => create_page(&page).await,
        Action::Submit => submit_page(&page, &submit).await,
    }
}

async fn view_page(page: &str) -> Result<Response, Error> {
    let db_url = std::env::var("DATABASE_URL")?;
    let pool = connect_database(&db_url).await?;

    let select_query = sqlx::query!(
        "SELECT page_id, page_title FROM pages WHERE page_title = $1",
        page
    );

    let result = select_query.fetch_one(&pool).await;
    match result {
        Ok(record) => Ok(Response::new(
            format!("Found page: '{}'", record.page_title).into(),
        )),
        Err(e) => match e {
            sqlx::Error::RowNotFound => page_not_found(&page).await,
            e => Err(e.into()),
        },
    }
}

async fn create_page(page: &str) -> Result<Response, Error> {
    let handlebars = Handlebars::new();
    Response::builder()
        .header("Content-Type", "text/html")
        .body(
            handlebars
                .render_template(
                    std::fs::read_to_string("./assets/templates/create-page.handlebars")?.as_str(),
                    &json!({
                        "page": page
                    }),
                )
                .expect("Handlebar threw an error!")
                .into(),
        )
        .map_err(|e| e.into())
}

async fn submit_page(page: &str, submit: &PageSubmit) -> Result<Response, Error> {
    let db_url = std::env::var("DATABASE_URL")?;
    let pool = connect_database(&db_url).await?;

    let select_query = sqlx::query!(
        "SELECT page_id, page_title FROM pages WHERE page_title = $1",
        page
    );
    let result = select_query.fetch_one(&pool).await;
    let query = match result {
        Ok(record) => {
            sqlx::query!(
                "UPDATE pages SET page_title = $1 WHERE page_id = $2",
                submit.title,
                record.page_id
            )
        }
        Err(e) => match e {
            sqlx::Error::RowNotFound => {
                sqlx::query!("INSERT INTO pages(page_title) VALUES ($1)", submit.title)
            }
            e => return Err(e.into()),
        },
    };

    _ = query.execute(&pool).await?;
    view_page(page).await
}

async fn page_not_found(page: &str) -> Result<Response, Error> {
    let handlebars = Handlebars::new();
    Response::builder()
        .header("Content-Type", "text/html")
        .body(
            handlebars
                .render_template(
                    std::fs::read_to_string("./assets/templates/not-found.handlebars")?.as_str(),
                    &json!({
                        "page": page
                    }),
                )
                .expect("Handlebar threw an error!")
                .into(),
        )
        .map_err(|e| e.into())
}
