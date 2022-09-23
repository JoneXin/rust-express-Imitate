use std::{collections::HashMap, fs::read_to_string, io, path::Path};

use serde_json::{json, Value};

use super::ServerConfig;

pub fn get_config() -> Value {
    // 链式调用
    let s = read_to_string("../../config/config.json").expect("找不到配置文件!!");
    json_to_struct(s)
}

/**
 * 字符串json 转结构体
 */
pub fn json_to_struct(str: String) -> Value {
    println!("{}", str);
    let v: Value = json!(str);
    return v;
}

pub fn parse_http_param() {}
