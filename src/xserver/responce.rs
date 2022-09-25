use std::{collections::HashMap, io::Write, net::TcpStream};

use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::{self, json};

pub struct HttpResponce {
    tcp_stream: Box<TcpStream>,
    res_line: String,
    res_header: String,
    res_data: String,
    res_header_map: HashMap<String, String>,
}

impl HttpResponce {
    pub fn new(tcp_stream: Box<TcpStream>) -> HttpResponce {
        let header_map = Self::set_default_header();
        HttpResponce {
            tcp_stream,
            res_line: String::from("HTTP/1.1 200 OK"),
            res_header: String::from(""),
            res_data: String::from(""),
            res_header_map: header_map,
        }
    }

    pub fn set_crross_domain(&mut self) {
        self.res_header = format!(
            "{}\r\n{}\r\n{}\r\n{}\r\n",
            self.res_header,
            "Access-Control-Allow-Origin: *",
            "Access-Control-Allow-Headers: X-Requested-With",
            "Access-Control-Allow-Methods: *"
        )
    }

    pub fn set_header(&mut self, header: &str, value: &str) {
        self.res_header_map
            .insert(String::from(header), String::from(value));
    }

    pub fn send(&mut self, data: &str) {
        self.res_data = String::from(data);
        self.res_header = self.transform_headermap_to_str();
        self.tcp_stream
            .write(
                format!(
                    "{}\r\n{}\r\n{}",
                    self.res_line, self.res_header, self.res_data
                )
                .as_bytes(),
            )
            .unwrap();
    }

    pub fn json<'a, R: ?Sized>(&mut self, data: R)
    where
        R: Serialize + Deserialize<'a>,
    {
        let john = json!(data);
        self.res_data = serde_json::to_string(&john).unwrap();
        self.res_header = self.transform_headermap_to_str();
        self.tcp_stream
            .write(
                format!(
                    "{}\r\n{}\r\n{}",
                    self.res_line, self.res_header, self.res_data
                )
                .as_bytes(),
            )
            .unwrap();
    }

    fn set_default_header() -> HashMap<String, String> {
        let mut header_map: HashMap<String, String> = HashMap::new();
        header_map.insert(String::from("X-Powered-By"), String::from("rust-server"));
        header_map.insert(String::from("Accept-Ranges"), String::from("bytes"));
        header_map.insert(
            String::from("Cache-Control"),
            String::from("public, max-age=0"),
        );
        header_map.insert(
            String::from("Content-Type"),
            String::from("application/json; charset=utf-8"),
        );
        header_map.insert(String::from("Connection"), String::from("close"));
        header_map.insert(
            String::from("ETag"),
            String::from("W/'58-khDYij7wGE5soY4bsF1PMYvE96c'"),
        );
        header_map.insert(String::from("Date"), Utc::now().to_string());
        header_map
    }

    fn transform_headermap_to_str(&mut self) -> String {
        let mut header_str = String::from("");

        for (key, value) in self.res_header_map.iter() {
            header_str = format!("{}{}: {}\r\n", header_str, key, value);
        }

        header_str
    }
}
