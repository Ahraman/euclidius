use axum::response::Response;

use crate::error::Error;

pub enum Req {
    Page(PageReq),
}

pub trait ReqExec {
    fn execute() -> Result<Response, Error>;
}

pub struct PageReq {
    pub title: String,
    pub kind: PageReqKind,
}

pub enum PageReqKind {
    View(ViewPageReq),
    Submit(SubmitPageReq),
    Edit(EditPageReq),
}

pub struct ViewPageReq {}

impl ReqExec for ViewPageReq {
    fn execute() -> Result<Response, Error> {
        todo!()
    }
}

pub struct SubmitPageReq {}

impl ReqExec for SubmitPageReq {
    fn execute() -> Result<Response, Error> {
        todo!()
    }
}

pub struct EditPageReq {}

impl ReqExec for EditPageReq {
    fn execute() -> Result<Response, Error> {
        todo!()
    }
}
