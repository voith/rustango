use basic_web_server::{
    server::Server,
    urls::get_routes
};


fn main() {
    let mut ser = Server::new();
    ser.register_routes(get_routes());
    ser.start(None);
}
