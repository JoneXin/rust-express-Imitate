use self::{responce::HttpResponce, router::XRouter};
use crate::HttpHeader;

pub mod app;
pub mod responce;
pub mod router;
pub mod threadpool;

pub type HttpHandler = fn(header: HttpHeader, responce: HttpResponce);
pub type RouterHander = fn(xrouter: &mut XRouter);
