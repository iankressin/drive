mod tcp_server;
mod udp_server;
mod utils;

use std::thread;

extern crate pnet;

fn main() -> Result<(), std::io::Error> {
    thread::spawn(|| {
        let mdns = udp_server::UdpServer::new();
        mdns.listen().unwrap();
    });

    let tcp_server = tcp_server::TcpServer::new();
    tcp_server.listen().unwrap();

    Ok(())
}
