use http::handler::HttpHandler;

mod http;
mod tcp_server;

fn main() {
    let server_config = tcp_server::Config::new("localhost".into(), 8080);
    let http_handler = HttpHandler::new();

    let server = tcp_server::Server::new(server_config, http_handler);

    server.run();
}
