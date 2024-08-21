use std::collections::HashMap;

use axum::{
    extract::{Query, State},
    response::{IntoResponse, Response},
};

use crate::{
    app::App,
    req::{Req, ReqExec},
    result::Result,
};

pub async fn get(
    State(app): State<App>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<Response> {
    match query.get("file") {
        Some(file) => Req::load(file).exec(app).await,
        None => Ok("".into_response()),
    }
}
