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

pub struct Server {
    routes: HashMap<String, BoxedView>,
}

impl Server {
    pub fn new() -> Self {
        let routes: HashMap<String, BoxedView> = HashMap::new();
        Server { routes }
    }

    pub fn start(self: Arc<Self>, port: Option<i32>, pool_size: Option<usize>) {
        // add default size of 1
        let size = pool_size.unwrap_or(1);
        let listener = match port {
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

    pub fn register_routes(&mut self, routes: Vec<(&'static str, BoxedView)>) {
        routes.into_iter().for_each(|(path, view)| {
            self.register_route(path, view);
        });
    }

    pub fn register_route(&mut self, route: &str, handler: BoxedView) {
        self.routes.insert(route.to_string(), handler);
    }

    fn get_handler(&self, route: &str) -> Option<&BoxedView> {
        self.routes.get(route)
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
