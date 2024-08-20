use std::sync::Arc;

use axum::{routing, Router};
use handlebars::Handlebars;
use sqlx::PgPool;

mod route;
use route::{page, root};

use crate::result::Result;

#[derive(Clone)]
pub struct App {
    inner: Arc<InnerApp>,
}

struct InnerApp {
    db_conn: PgPool,
    handlebars: Handlebars<'static>,
}

impl App {
    pub fn new(db_conn: PgPool) -> Result<Self> {
        Ok(Self {
            inner: Arc::new(InnerApp {
                db_conn,
                handlebars: Self::create_handlebars()?,
            }),
        })
    }

    pub fn db_conn(&self) -> &PgPool {
        &self.inner.db_conn
    }

    pub fn handlebars(&self) -> &Handlebars<'static> {
        &self.inner.handlebars
    }

    pub fn into_router(self) -> Router {
        Router::new()
            .route("/:title", routing::get(page::get).post(page::set))
            .route("/", routing::get(root::get))
            .with_state(self)
    }
}

impl App {
    fn create_handlebars() -> Result<Handlebars<'static>> {
        let mut handlebars = Handlebars::new();
        handlebars
            .register_template_file("create-page", "./assets/templates/create-page.handlebars")?;
        handlebars
            .register_template_file("not-found", "./assets/templates/not-found.handlebars")?;

        Ok(handlebars)
    }
}
