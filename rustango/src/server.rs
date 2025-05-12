use super::http::{RequestErrors, Response, parse_request};
use super::views::BoxedView;
use super::thread_pool::ThreadPool;
use httparse::{EMPTY_HEADER, Request};
use std::{
    collections::HashMap,
    io::prelude::*,
    net::{TcpListener, TcpStream},
    sync::Arc,
};
pub struct ServerConfig {
    pub routes: HashMap<String, BoxedView>,
    pub port: Option<i32>,
    pub pool_size: Option<usize>
}

pub struct Server {
    config: ServerConfig
}

impl Server {
    pub fn new(config: ServerConfig) -> Self {
        Self { config }
    }

    pub fn start(self: Arc<Self>) {
        // add default size of 1
        let size = self.config.pool_size.unwrap_or(1);
        let listener = match self.config.port {
            Some(_port) => {
                let address = format!("127.0.0.1:{_port}");
                println!("address: {}", address);
                TcpListener::bind(address).unwrap()
            }
            None => TcpListener::bind("127.0.0.1:0").unwrap(),
        };
        let addr = listener.local_addr().unwrap();
        let thread_pool = ThreadPool::new(size);
        println!("started server. Vist http://{}", addr);
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let server = Arc::clone(&self);
            thread_pool.execute(move ||{
                server.handle_request(stream);
            });
        }
    }

    fn get_handler(&self, route: &str) -> Option<&BoxedView> {
        self.config.routes.get(route)
    }

    pub fn handle_request(&self, mut stream: TcpStream) {
        let mut buf = [0; 1024];
        let mut headers = [EMPTY_HEADER; 16];
        let request = parse_request(&mut buf, &mut headers, &mut stream);
        let response: Response;
        match request {
            Ok(req) => {
                response = self.generate_response(req);
            }
            Err(RequestErrors::ParsingError) => {
                println!("request parsing error");
                response = Response {
                    data: "".to_string(),
                    status_code: 400,
                    version: "1.1",
                };
            }
        }

        stream.write_all(response.to_string().as_bytes()).unwrap();
    }

    fn generate_response(&self, request: Request) -> Response {
        let request_method = request.method.unwrap();
        let request_path = request.path.unwrap();
        println!("received request {}, path={}", request_method, request_path);

        match self.get_handler(request_path) {
            Some(view) => match request_method {
                "GET" => view.get(),
                "POST" => view.post(),
                "PUT" => view.put(),
                "PATCH" => view.patch(),
                "DELETE" => view.delete(),
                "HEAD" => view.head(),
                "OPTIONS" => view.options(),
                _ => view.default_response(),
            },
            None => Response {
                data: "<!DOCTYPE html><html lang='en'>404 Not Found</html>".to_string(),
                status_code: 404,
                version: "1.1",
            },
        }
    }
}
