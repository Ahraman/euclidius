use std::collections::HashMap;

use axum::{
    extract::{Path, Query, State},
    response::Response,
    Form,
};

use crate::{
    app::App,
    error::Error,
    req::{Req, ReqExec, SubmitPage},
    result::Result,
};

pub async fn get(
    State(app): State<App>,
    Path(title): Path<String>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<Response> {
    match query.get("action").map(|s| s.as_str()) {
        Some("view") | None => Req::view_page(title).exec(app).await,
        Some("edit") => Req::edit_page(title).exec(app).await,
        Some("submit") => Req::submit_page(title, None).exec(app).await,
        Some(value) => Err(Error::BadReq("action".to_string(), value.to_string())),
    }
}

pub async fn set(
    State(app): State<App>,
    Path(title): Path<String>,
    Query(query): Query<HashMap<String, String>>,
    Form(content): Form<HashMap<String, String>>,
) -> Result<Response> {
    match query.get("action").map(|s| s.as_str()) {
        Some("view") | None => Req::view_page(title).exec(app).await,
        Some("edit") => Req::edit_page(title).exec(app).await,
        Some("submit") => {
            Req::submit_page(
                title,
                Some(SubmitPage::new(
                    content
                        .get("content")
                        .cloned()
                        .unwrap_or("THIS IS DEFAULT CONTENT".to_string()),
                    1,
                )),
            )
            .exec(app)
            .await
        }
        Some(value) => Err(Error::BadReq("action".to_string(), value.to_string())),
    }
}
