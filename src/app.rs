use axum::{routing::get, Router};
use sqlx::PgPool;

pub mod route;
use self::route::{page, root};

#[derive(Clone)]
pub struct App {
    pub db_conn_pool: PgPool,
}

impl App {
    pub fn new(conn: PgPool) -> Self {
        Self { db_conn_pool: conn }
    }

    pub fn build_router(self) -> Router {
        Router::new()
            .route("/", get(root::get))
            .route("/:page", get(page::get).post(page::post))
            .with_state(self)
    }
}
