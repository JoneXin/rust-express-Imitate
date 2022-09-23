use crate::HttpHeader;
use std::{io::Write, net::TcpStream};

pub struct User {}

impl User {
    pub fn add_user(header: &HttpHeader, responce: &mut TcpStream) {
        println!("{:#?}", header.body);
        responce.write(b"HTTP/1.1 200 OK").unwrap();
        responce.flush().unwrap();
    }

    // 获取用户信息
    pub fn get_user(header: &HttpHeader, responce: &mut TcpStream) {
        println!("{:#?}", header.request_param);

        responce.write("HTTP/1.1 200 OK".as_bytes()).unwrap();
        responce.flush().unwrap();
    }
}
