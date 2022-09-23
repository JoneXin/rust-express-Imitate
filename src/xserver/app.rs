use super::responce::HttpResponce;
use super::router::XRouter;
use super::{HttpHandler, RouterHander};
use crate::{HttpHeader, HttpMethod, HttpVersion};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;
use std::collections::HashMap;
use std::error::Error;
use std::io::Write;
use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

const REQUEST_SIZE: usize = 1024;
pub struct App {
    xrouter: XRouter,
}

impl App {
    pub fn new() -> App {
        // 配置
        App {
            xrouter: XRouter {
                route: String::from("/"),
                router_table: HashMap::new(),
            },
        }
    }

    pub fn router(&mut self, route: &str, func: RouterHander) {
        // 一级路由
        self.xrouter.route = String::from(route);
        // 调用外部定义处理路由函数
        func(&mut self.xrouter);
    }

    pub fn listen(&mut self, addr: &str) -> Result<(), Box<dyn Error>> {
        let listener = TcpListener::bind(addr)?;

        for steam in listener.incoming() {
            let streams = &mut steam.unwrap();
            self.handle_request(streams);
        }

        Ok(())
    }

    fn handle_request(&mut self, steam: &mut TcpStream) {
        let mut buffer = [0; REQUEST_SIZE];
        steam.read(&mut buffer).unwrap();
        let request_str = String::from_utf8_lossy(&buffer);
        // 解析header
        let header = self.parse_header(request_str);
        // 调用相应的处理函数
        self.distribute_handler(&header, steam);
    }

    fn parse_header(&mut self, req: Cow<str>) -> HttpHeader {
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
            println!("{}", request_line);
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

    fn distribute_handler(&mut self, header: &HttpHeader, res_handler: &mut TcpStream) {
        let method_str = if let HttpMethod::GET = header.method {
            "get"
        } else {
            "post"
        };
        let xrouter_map_key = format!("{}{}", header.reuqest_addr, method_str);

        let handler = self.xrouter.router_table.get(&xrouter_map_key[..]);
        let mut http_responce_handler = HttpResponce::new(res_handler);

        // 路由表有定义
        if let Some(handle_func) = handler {
            handle_func(header, res_handler);
        } else {
            // 没定义返回404
            self.handle404(header, &mut http_responce_handler);
        }
    }

    fn handle404(&mut self, _: &HttpHeader, res_handler: &mut HttpResponce) {
        println!("{}", "404了亲!");

        // let aa = ResponceMessage {
        //     code: 200,
        //     message: "success",
        //     data: vec![1, 222222, 1231123123],
        // };
        res_handler.set_header("Content-Type", "text/html; charset=utf-8");
        res_handler.send(
            r#"<!DOCTYPE html>
        <html lang="en">
        
        <head>
            <meta charset="UTF-8">
            <meta http-equiv="X-UA-Compatible" content="IE=edge">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Document</title>
        </head>
        
        <body>
            <html>吴斌最帅
        
            </html>
        </body>
        
        </html>"#,
        );
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
