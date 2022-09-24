use std::{
    f32::consts::E,
    sync::Arc,
    thread::{self, JoinHandle, Thread},
};

use crate::HttpHeader;

use super::{responce::HttpResponce, HttpHandler};

pub struct HttpHadnlerThreadPool {
    thead_max: usize,
}

impl HttpHadnlerThreadPool {
    pub fn new(thead_max: usize) -> HttpHadnlerThreadPool {
        HttpHadnlerThreadPool { thead_max }
    }

    pub fn exec(&self, f: HttpHandler, header: HttpHeader, responce: HttpResponce) {
        let t = thread::spawn(move || {
            println!("{}", "我是处理线程");
            f(header, responce)
        });
        println!("{}", "我好了，你呢 ");
        t.join().unwrap();
    }
}
