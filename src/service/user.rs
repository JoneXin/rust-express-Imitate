use crate::{xserver::responce::HttpResponce, HttpHeader};

pub struct User {}

impl User {
    pub fn add_user(header: HttpHeader, mut responce: HttpResponce) {
        println!("{:#?}", header.body);
        // 设置头部
        responce.set_header("Content-Type", "text/pain; charset=utf-8");
        // 返回也该普通文本
        responce.send("a");
    }

    // 获取用户信息
    pub fn get_user(header: HttpHeader, mut responce: HttpResponce) {
        println!("{:#?}", header.request_param);
        // 默认application/json
        responce.send("b");
    }
}
