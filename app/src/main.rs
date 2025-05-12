use app::urls::get_routes;
use rustango::server::{Server, ServerConfig};
use std::sync::Arc;

fn main() {
    let config = ServerConfig {
        routes: get_routes(),
        port: Some(8080),
        pool_size: Some(4)
    };
    let server = Arc::new(Server::new(config));
    server.start();
}
