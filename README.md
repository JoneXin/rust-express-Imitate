## 项目

### 用rust实现了一个类似express的web服务器(基础款)

### 在此基础上，使用线程池处理请求，实现又express又rust

## 功能

### 1，封装了路由表，应用层面直接类似express 路由调用
mian.rs
```rust
use web_server::router::Router;
use web_server::xserver::app::App;
fn main() {
    let mut app = App::new();
    app.router("/user", Router::user_router);
    app.listen("0.0.0.0:4444").unwrap();
}

```

router/mod.rs

```rust
use crate::server::user::User;
use crate::xserver::router::XRouter;
pub struct Router {}

impl Router {
    pub fn user_router(x_router: &mut XRouter) {
        x_router.get("/query", User::get_user);
        x_router.post("/add", User::add_user);
    }
}

```
server/user.rs
```rust
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


```

### 2，封装了 解析请求头，构造响应头等API

### 3，封装了线程池

## 项目结构（主要的）

```
--/src
    | -- router/ 路由
    |
    | -- service/ 业务逻辑
    |
    | -- xserver/ 封装的web-server（可以单独拎出来，我就不拎了）
    |       |
    |       | --app 应用
    |       | --responce 封装的响应头
    |       | --router 封装的应用层注册的路由表（hashmap存储）
    |       | --threadpool 封装 处理web请求的线程池
```

## 说明

### 这个项目写的原因是 读《rust程序设计语言》最后有个实现高并发web服务器的章节，在此基础上 实践rust，旨在学习


## 运行

```rust
cargo install
cargo run
```