use std::{
    collections::HashMap,
    io::prelude::*,
    net::{TcpListener, TcpStream},
};
use httparse::{EMPTY_HEADER, Header, Request, Status};

pub struct Response {
    pub data: String,
    pub status_code: u16,
    pub version: &'static str
}

enum RequestErrors {
    ParsingError
}

pub trait View {
    fn get(&self) -> Response {
        self.default_response()
    }

    fn post(&self) -> Response {
        self.default_response()
    }

    fn put(&self) -> Response {
        self.default_response()
    }

    fn patch(&self) -> Response {
        self.default_response()
    }

    fn delete(&self) -> Response {
        self.default_response()
    }

    fn head(&self) -> Response {
        self.default_response()
    }

    fn options<'a>(&self) -> Response {
        self.default_response()
    }

    fn default_response(&self) -> Response {
        Response{data: "<!DOCTYPE html><html lang='en'>405 Method Not Allowed</html>".to_string(), status_code: 405, version: "1.1"}
    }
}

pub type BoxedView = Box<dyn View + Send + Sync>;

pub struct Server {
    routes: HashMap<String, BoxedView>
}

impl Response {
    
    pub fn to_string(&self) -> String {
        let data = &self.data;
        let status_code_str = self.status_code_to_string();
        let version_str = self.version_to_string();
        let length = data.len();
        let res = format!("{version_str} {status_code_str}\r\nContent-Length: {length}\r\n\r\n{data}");
        res
    }

    fn version_to_string(&self) -> String {
        match self.version {
            "1.1" => "HTTP/1.1".to_string(),
            _ => "HTTP/1.0".to_string(), // fallback
        }
    }

    fn status_code_to_string(&self) -> String {
        match self.status_code {
            200 => "200 OK".to_string(),
            400 => "400 Bad request".to_string(),
            404 => "404 NOT FOUND".to_string(),
            405 => "405 Method Not Allowed".to_string(),
            i => format!("{i} UNKOWN STATUS CODE")
        }
    }
}

impl Server {

    pub fn new() ->  Self {
        let routes: HashMap<String, BoxedView> = HashMap::new();
        Server{ routes}
    }

    pub fn start(&self, port: Option<i32>) {
        let listener = match port {
            Some(_port) => {
                let address = format!("127.0.0.1:{_port}");
                println!("address: {}", address);
                TcpListener::bind(address).unwrap()
            },
            None => {
                TcpListener::bind("127.0.0.1:0").unwrap()
            }
        };
        let addr = listener.local_addr().unwrap();
        println!("started server. Vist http://{}", addr);
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            self.handle_request(stream);
        }
    }

    pub fn register_routes(&mut self, routes: Vec<(&'static str, BoxedView)>) {
        routes.into_iter().for_each(|(path, view)| {
            self.register_route(path, view);
        });
    }

    pub fn register_route(&mut self, route: &str, handler: BoxedView ) {
        self.routes.insert(route.to_string(), handler);
    }

    fn get_handler(&self, route: &str) -> Option<&BoxedView> {
        self.routes.get(route)
    }

    pub fn handle_request(&self, mut stream: TcpStream) {
        let mut buf = [0; 1024];
        let mut headers = [EMPTY_HEADER; 16];
        let request = self.parse_request(
            &mut buf,
            &mut headers,
            &mut stream
        );
        let response :Response;
        match request {
            Ok(req) => {
                response = self.generate_response(req);
            }
            Err(RequestErrors::ParsingError) => {
                println!("request parsing error");
                response = Response {
                    data: "".to_string(),
                    status_code: 400,
                    version: "1.1"
                };
            }
        }
        
        stream.write_all(response.to_string().as_bytes()).unwrap();
    }

    fn generate_response(&self, request: Request) -> Response {
        let request_method = request.method.unwrap();
        let request_path = request.path.unwrap();
        println!(
            "received request {}, path={}", 
            request_method, 
            request_path
        );

        match self.get_handler(request_path) {
            Some(view) => {
                match request_method {
                    "GET" => view.get(),
                    "POST" => view.post(),
                    "PUT" => view.put(),
                    "PATCH" => view.patch(),
                    "DELETE" => view.delete(),
                    "HEAD" => view.head(),
                    "OPTIONS" => view.options(),
                    _ => view.default_response()
                }
            },
            None => Response{data: "<!DOCTYPE html><html lang='en'>404 Not Found</html>".to_string(), status_code: 404, version: "1.1"}
        }
    }

    fn parse_request<'a>(
        &self, 
        buf: &'a mut [u8],
        headers: &'a mut [Header<'a>],
        stream: &'a mut TcpStream
    ) -> Result<Request<'a,'a>, RequestErrors> {
        stream.read(buf).unwrap();
        let mut req = Request::new(headers);
        match req.parse(buf) {
            Ok(Status::Complete(_)) => {
                Ok(req)
            }
            _ => {
                Err(RequestErrors::ParsingError)
            }
        }
        
    }
}
