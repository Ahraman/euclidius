use axum::{extract::Path, response::IntoResponse};

pub async fn get(Path(page): Path<String>) -> impl IntoResponse {
    format!("Looking for page '{page}'.")
}
