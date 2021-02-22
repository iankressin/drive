mod tcp_server;
mod types;
mod udp_server;
mod utils;

use std::thread;

extern crate pnet;

fn main() -> Result<(), std::io::Error> {
    thread::spawn(|| {
        let mut meta_list = vec![types::Metadata {
            name: "fuji".to_string(),
            extension: "jpg".to_string(),
            name_extension: "fuji.jpg".to_string(),
            hash: "b0e490e762234567ebc74fade854476fe692e320".to_string(),
            size: 124093,
        }];
        let mut tcp_server = tcp_server::TcpServer::new(&mut meta_list);
        tcp_server.listen().unwrap();
    });

    let mdns = udp_server::UdpServer::new();
    mdns.listen().unwrap();

    Ok(())
}
