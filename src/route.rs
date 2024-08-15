use axum::routing::get;

pub mod page;
pub mod root;

pub fn build_router() -> axum::Router {
    axum::Router::new()
        .route("/", get(root::get))
        .route("/:page", get(page::get).post(page::post))
}
