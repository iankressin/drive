mod tcp_server;
mod udp_server;
mod utils;

extern crate pnet;

fn main() -> Result<(), std::io::Error> {
    // let mdns = udp_server::UdpServer::new();
    // mdns.listen()?;
    let tcp_server = tcp_server::TcpServer::new();
    tcp_server.listen()?;
    Ok(())
}
