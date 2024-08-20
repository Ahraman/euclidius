use std::future::Future;

use axum::{
    body::Body,
    response::{IntoResponse, Response},
};
use serde_json::json;

use crate::{app::App, error::Error, result::Result};

pub struct PageContent {
    pub text: String,
    pub page_id: i32,
    pub page_rev: i32,
}

#[derive(Clone)]
pub struct SubmitPage {
    text: String,
    user_id: i32,
}

impl SubmitPage {
    pub fn new(text: String, user_id: i32) -> Self {
        Self { text, user_id }
    }

    pub async fn submit(
        self,
        title: String,
        app: App,
        content: Option<PageContent>,
    ) -> Result<Response> {
        const INSERT_REVISION: &'static str =
            "INSERT INTO revisions (rev_parent, rev_content, rev_page, \
            rev_user, rev_timestamp) VALUES ($1, $2, $3, $4, $5) \
            RETURNING rev_id";

        const INSERT_PAGE: &'static str =
            "INSERT INTO pages (page_title, page_rev) VALUES ($1, $2) RETURNING page_id";

        const UPDATE_PAGE: &'static str =
            "UPDATE pages SET page_rev = $2 WHERE page_title = $1 RETURNING page_id";

        const UPDATE_REVISION: &'static str =
            "UPDATE revisions SET rev_page = $2 WHERE rev_id = $1";

        let conn = app.db_conn();

        let page_title = &title;
        let (rev_page, rev_parent) = content.map(|v| (v.page_id, v.page_rev)).unzip();
        let (rev_content, rev_user) = (self.text, self.user_id);
        let rev_timestamp = time::OffsetDateTime::now_utc();

        let page_rev: i32 = sqlx::query_scalar(INSERT_REVISION)
            .bind(rev_parent)
            .bind(rev_content)
            .bind(rev_page)
            .bind(rev_user)
            .bind(rev_timestamp)
            .fetch_one(conn)
            .await?;

        let query_text = match rev_page {
            Some(_) => UPDATE_PAGE,
            None => INSERT_PAGE,
        };
        let rev_page: i32 = sqlx::query_scalar(query_text)
            .bind(page_title)
            .bind(page_rev)
            .fetch_one(conn)
            .await?;

        let rev_id = page_rev;
        let _ = sqlx::query(UPDATE_REVISION)
            .bind(rev_id)
            .bind(rev_page)
            .execute(conn)
            .await?;

        Ok(format!(
            "Submitted {page_title} successfully with rev_id={rev_id} and page_id={rev_page}"
        )
        .into_response())
    }
}

#[derive(Clone)]
pub enum PageReqKind {
    View,
    Edit,
    Submit(Option<SubmitPage>),
}

pub struct PageReq {
    title: String,
    kind: PageReqKind,
}

impl PageReq {
    async fn exec(self, app: App) -> Result<Response> {
        const SELECT_PAGE: &'static str =
            "SELECT page_id, page_rev FROM pages WHERE page_title = $1";
        const SELECT_REVISION: &'static str = "SELECT rev_content FROM revisions WHERE rev_id = $1";

        let conn = app.db_conn();

        let content = {
            let page: Option<(i32, i32)> = sqlx::query_as(SELECT_PAGE)
                .bind(&self.title)
                .fetch_optional(conn)
                .await?;

            match page {
                Some((page_id, page_rev)) => {
                    let rev_content: String = sqlx::query_scalar(SELECT_REVISION)
                        .bind(page_rev)
                        .fetch_one(conn)
                        .await?;

                    Some(PageContent {
                        text: rev_content,
                        page_id,
                        page_rev,
                    })
                }
                None => None,
            }
        };

        match self.kind {
            PageReqKind::View => self.exec_view(app, content).await,
            PageReqKind::Edit => self.exec_edit(app, content).await,
            PageReqKind::Submit(_) => self.exec_submit(app, content).await,
        }
    }

    async fn exec_view(self, app: App, content: Option<PageContent>) -> Result<Response> {
        match content {
            Some(content) => {
                Ok(format!("Found page: {}\n\n{}", &self.title, &content.text).into_response())
            }
            None => self.page_not_found(app).await,
        }
    }

    async fn exec_edit(self, app: App, _: Option<PageContent>) -> Result<Response> {
        Response::builder()
            .header("Content-Type", "text/html")
            .body(Body::from(
                app.handlebars().render("create-page", &json!({}))?,
            ))
            .map_err(|e| e.into())
    }

    async fn exec_submit(self, app: App, content: Option<PageContent>) -> Result<Response> {
        match self.kind {
            PageReqKind::Submit(Some(post)) => post.submit(self.title, app, content).await,
            PageReqKind::Submit(None) => self.exec_view(app, content).await,
            _ => Err(Error::InvalidReq),
        }
    }

    async fn page_not_found(self, app: App) -> Result<Response> {
        Response::builder()
            .header("Content-Type", "text/html")
            .body(Body::from(
                app.handlebars().render("not-found", &json!({}))?,
            ))
            .map_err(|e| e.into())
    }
}

pub enum Req {
    None,
    Page(PageReq),
}

impl Req {
    pub fn view_page(title: impl Into<String>) -> Self {
        Self::Page(PageReq {
            title: title.into(),
            kind: PageReqKind::View,
        })
    }

    pub fn edit_page(title: impl Into<String>) -> Self {
        Self::Page(PageReq {
            title: title.into(),
            kind: PageReqKind::Edit,
        })
    }

    pub fn submit_page(title: impl Into<String>, post: Option<SubmitPage>) -> Self {
        Self::Page(PageReq {
            title: title.into(),
            kind: PageReqKind::Submit(post),
        })
    }
}

pub trait ReqExec {
    fn exec(self, app: App) -> impl Future<Output = Result<Response>> + Send;
}

impl ReqExec for Req {
    async fn exec(self, app: App) -> Result<Response> {
        match self {
            Req::None => Err(Error::NoReq),
            Req::Page(req) => req.exec(app).await,
        }
    }
}
