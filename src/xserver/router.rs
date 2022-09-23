use std::collections::HashMap;

use crate::xserver::HttpHandler;

#[derive(Clone)]
pub struct XRouter {
    pub route: String,
    pub router_table: HashMap<String, HttpHandler>,
}

impl XRouter {
    pub fn new(route: String) -> XRouter {
        XRouter {
            route,
            router_table: HashMap::new(),
        }
    }

    // 注册get请求和处理函数
    pub fn get(&mut self, c_route: &str, f: HttpHandler) {
        self.router_table
            .insert(format!("{}{}{}", self.route, c_route, "get"), f);
    }

    // 注册post请求和函数
    pub fn post(&mut self, c_route: &str, f: HttpHandler) {
        self.router_table
            .insert(format!("{}{}{}", self.route, c_route, "post"), f);
    }
}
