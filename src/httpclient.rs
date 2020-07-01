pub mod httpclient {
    use std::net::{TcpListener, TcpStream};
    use thread_pool::{Work, Threadpool};

    pub struct Httpserver {
        listener: TcpListener,
        threadpool: Threadpool,
    }

    struct Task {
        stream: TcpStream,
    }

    impl thread_pool::Process for Task {
        fn exec(&self) {
            println!("rec...");
        }
    }

    impl Httpserver {
        pub fn new(host: &str, threadnum: i64) -> Httpserver {
            println!("{}", host);
            let listener = TcpListener::bind(host).unwrap();
            let threadpool = Threadpool::new(threadnum as usize);

            Httpserver {
                listener,
                threadpool,
            }
        }

        pub fn run(&self) {
            for stream in self.listener.incoming() {
                self.handle(stream.unwrap());
            }
        }

        //type Job = Box<dyn Process + Send + 'static>;
        fn handle(&self, stream: TcpStream) {
            let w = Box::new(Task { stream });
            self.threadpool.send_task(w);
        }
    }
}