use app::urls::get_routes;
use rustango::server::Server;

fn main() {
    let mut ser = Server::new();
    ser.register_routes(get_routes());
    ser.start(None);
}
