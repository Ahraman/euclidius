use axum::{
    extract::Path,
    response::{IntoResponse, Redirect},
    routing::get,
};
use euclidius::error::Error;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv()?;

    let listener = TcpListener::bind("localhost:3000").await?;
    axum::serve(listener, build_router()).await?;

    Ok(())
}

pub fn build_router() -> axum::Router {
    axum::Router::new()
        .route("/", get(root))
        .route("/:page", get(show_page))
}

async fn root() -> impl IntoResponse {
    Redirect::to("/main")
}

async fn show_page(Path(page): Path<String>) -> impl IntoResponse {
    format!("Looking for page '{page}'.")
}
