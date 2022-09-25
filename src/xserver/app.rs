use super::responce::HttpResponce;
use super::router::XRouter;
use super::threadpool::HttpHadnlerThreadPool;
use super::{HttpHandler, RouterHander};
use crate::{HttpHeader, HttpMethod, HttpVersion};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::read_to_string;
use std::path::Path;
use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

const REQUEST_SIZE: usize = 1024;

pub struct App {
    xrouter: XRouter,
    http_thread_pool: HttpHadnlerThreadPool,
}

impl App {
    pub fn new() -> App {
        let http_thread_pool = HttpHadnlerThreadPool::new(20);
        // 配置
        App {
            xrouter: XRouter {
                route: String::from("/"),
                router_table: HashMap::new(),
            },
            http_thread_pool,
        }
    }

    pub fn router(&mut self, route: &str, func: RouterHander) {
        // 一级路由
        self.xrouter.route = String::from(route);
        // 调用外部定义处理路由函数
        func(&mut self.xrouter);
    }

    pub fn listen(&self, addr: &str) -> Result<(), Box<dyn Error>> {
        let listener = TcpListener::bind(addr)?;

        for steam in listener.incoming() {
            let route_map = self.xrouter.router_table.clone();
            self.http_thread_pool
                .exec(move || Self::handle_request(steam.unwrap(), route_map))
        }
        Ok(())
    }

    fn handle_request(mut steam: TcpStream, route_map: HashMap<String, HttpHandler>) {
        let mut buffer = [0; REQUEST_SIZE];
        steam.read(&mut buffer).unwrap();
        let request_str = String::from_utf8_lossy(&buffer);
        // 解析header
        let header = App::parse_header(request_str);
        // 调用相应的处理函数
        Self::distribute_handler(header, steam, route_map);
    }

    fn parse_header(req: Cow<str>) -> HttpHeader {
        let mut header = HttpHeader {
            method: crate::HttpMethod::GET,
            body: Value::Null,
            reuqest_addr: String::from(""),
            request_param: HashMap::new(),
            http_version: crate::HttpVersion::HTTP(String::from("")),
        };

        // parse header
        let request_addr = req.lines().filter(|line| line.contains("HTTP/")).next();
        if let Some(request_line) = request_addr {
            let temp_arr: Vec<&str> = request_line.split(" ").collect();
            let route: Vec<&str> = temp_arr[1].split("?").collect();
            // 有参数
            if route.len() > 1 {
                let param = App::parse_param(&route[1]);
                header.request_param = param;
            }
            header.reuqest_addr = String::from(route[0]);
            header.http_version = HttpVersion::HTTP(String::from(temp_arr[2]));
        }

        // method
        if req.starts_with("POST") {
            header.method = HttpMethod::POST;

            // body
            for (i, req_line) in req.lines().enumerate() {
                if req_line == "{" {
                    let body = App::get_request_body(req.clone(), i);
                    header.body = body;
                }
            }
        }

        header
    }

    fn distribute_handler(
        header: HttpHeader,
        res_handler: TcpStream,
        route_map: HashMap<String, HttpHandler>,
    ) {
        let method_str = if let HttpMethod::GET = header.method {
            "get"
        } else {
            "post"
        };
        let xrouter_map_key = format!("{}{}", header.reuqest_addr, method_str);

        let handler = route_map.get(&xrouter_map_key[..]);
        let http_responce_handler = HttpResponce::new(Box::new(res_handler));

        // 路由表有定义
        if let Some(handle_func) = handler {
            handle_func(header, http_responce_handler);
        } else {
            App::handle404(header, http_responce_handler);
        }
    }

    fn handle404(header: HttpHeader, mut res_handler: HttpResponce) {
        println!("{} 不存在此路由!", header.reuqest_addr);
        res_handler.set_header("Content-Type", "text/html; charset=utf-8");
        let cur_dir = env::current_dir().expect("not found path");
        let page_path = Path::new(&cur_dir).join("../public/404.html");
        let not_found_page = read_to_string(page_path).unwrap();
        res_handler.send(&not_found_page[..]);
    }

    fn parse_param(str: &str) -> HashMap<String, String> {
        let mut parma_map = HashMap::new();
        let kes_iter = str.split("&");
        for item in kes_iter {
            let key_value_vec: Vec<&str> = item.split("=").collect();
            parma_map.insert(
                String::from(key_value_vec[0]),
                String::from(key_value_vec[1]),
            );
        }
        parma_map
    }

    fn get_request_body(req: Cow<str>, i: usize) -> Value {
        let mut body_data: Vec<&str> = req.split("\r\n").map(|v| v.trim()).collect();
        body_data.pop();
        body_data.push("}");
        let str = format!(r#"{}"#, &body_data[i..].join(""));
        let body_struct: Value = serde_json::from_str(&str[..]).unwrap();
        return body_struct;
    }
}

#[derive(Serialize, Deserialize)]
struct ResponceMessage<'a, T> {
    code: i32,
    message: &'a str,
    data: T,
}
