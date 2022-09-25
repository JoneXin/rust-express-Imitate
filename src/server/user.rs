use crate::{xserver::responce::HttpResponce, HttpHeader};

pub struct User {}

impl User {
    pub fn add_user(header: HttpHeader, mut responce: HttpResponce) {
        responce.send("a");
    }

    // 获取用户信息
    pub fn get_user(header: HttpHeader, mut responce: HttpResponce) {
        println!("{:#?}", header.request_param);
        responce.send("b");
    }
}
