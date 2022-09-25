use std::{
    sync::{mpsc, Arc, Mutex},
    thread::{self},
};

type Job = Box<dyn FnOnce() + 'static + Send + Sync>;

pub enum Message {
    NewJob(Job),
    Terminate,
}

unsafe impl Sync for Message {}

pub struct Work {
    pub id: usize,
    pub thread: Option<thread::JoinHandle<()>>,
}

impl Work {
    pub fn new(id: usize, receive: Arc<Mutex<mpsc::Receiver<Message>>>) -> Work {
        // 创建线程 不结束，一直接受消息
        let t = thread::spawn(move || {
            println!("{}号线程创建成功！", id);
            loop {
                let msg = receive.lock().unwrap().recv().unwrap();

                match msg {
                    Message::NewJob(job) => {
                        job();
                    }
                    // 发送销毁线程信息
                    Message::Terminate => {
                        break;
                    }
                }
            }
        });

        Work {
            id,
            thread: Some(t),
        }
    }
}

pub struct HttpHadnlerThreadPool {
    pub sender: mpsc::Sender<Message>,
    pub works: Vec<Work>,
}

impl HttpHadnlerThreadPool {
    pub fn new(thead_size: usize) -> HttpHadnlerThreadPool {
        let (sd, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));
        let mut work = Vec::with_capacity(thead_size);

        for i in 0..thead_size {
            work.push(Work::new(i, Arc::clone(&rx)));
        }
        HttpHadnlerThreadPool {
            sender: sd,
            works: work,
        }
    }

    pub fn exec<F>(&self, f: F)
    where
        F: Send + 'static + FnOnce() + Sync,
    {
        self.sender.send(Message::NewJob(Box::new(f))).unwrap();
    }
}

impl Drop for HttpHadnlerThreadPool {
    fn drop(&mut self) {
        for _ in &self.works {
            self.sender.send(Message::Terminate).unwrap();
        }

        for work in &mut self.works {
            if let Some(thred) = work.thread.take() {
                thred.join().unwrap();
            }
        }
    }
}
