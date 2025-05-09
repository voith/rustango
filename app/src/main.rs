use rustango::server::Server;
use app::urls::get_routes;

fn main() {
    let mut ser = Server::new();
    ser.register_routes(get_routes());
    ser.start(None);
}
