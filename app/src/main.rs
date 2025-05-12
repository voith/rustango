use app::urls::get_routes;
use rustango::server::Server;
use std::sync::Arc;

fn main() {
    let mut ser = Server::new();
    ser.register_routes(get_routes());
    let ser = Arc::new(ser);
    ser.start(Some(8080), Some(4));
}
