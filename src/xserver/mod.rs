use self::router::XRouter;
use crate::HttpHeader;
use std::net::TcpStream;

pub mod app;
pub mod responce;
pub mod router;
pub type HttpHandler = fn(header: &HttpHeader, responce: &mut TcpStream);
pub type RouterHander = fn(xrouter: &mut XRouter);
