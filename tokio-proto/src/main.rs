extern crate tokio_proto;

use self::tokio_proto::TcpServer;

mod codec;
mod protocol;
mod service;

fn main() {
    let addr = "0.0.0.0:12345".parse().unwrap();
    let server = TcpServer::new(protocol::LineProto, addr);

    server.serve(|| Ok(service::EchoService));
}