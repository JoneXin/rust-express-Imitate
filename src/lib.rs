use std::collections::HashMap;

use serde_json::Value;

pub mod router;
pub mod server;
pub mod utils;
pub mod xserver;

#[derive(Debug)]
pub enum HttpMethod {
    GET,
    POST,
}
#[derive(Debug)]
pub enum HttpVersion {
    HTTP(String),
}

#[derive(Debug)]
pub struct HttpHeader {
    pub method: HttpMethod,
    pub body: Value,
    pub reuqest_addr: String,
    pub request_param: HashMap<String, String>,
    pub http_version: HttpVersion,
}
