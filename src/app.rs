use std::sync::Arc;

use axum::{routing, Router};
use handlebars::Handlebars;
use sqlx::PgPool;

mod route;
use route::{load, page, root};

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
            .route("/wiki/:title", routing::get(page::get).post(page::set))
            .route("/wiki/", routing::get(root::get))
            .route("/load", routing::get(load::get))
            .with_state(self)
    }
}

impl App {
    fn create_handlebars() -> Result<Handlebars<'static>> {
        let mut handlebars = Handlebars::new();
        handlebars
            .register_template_file("index", "./assets/euclidius/templates/index.handlebars")?;
        handlebars.register_template_file(
            "create-page",
            "./assets/euclidius/templates/create-page.handlebars",
        )?;
        handlebars.register_template_file(
            "not-found",
            "./assets/euclidius/templates/not-found.handlebars",
        )?;

        Ok(handlebars)
    }
}
