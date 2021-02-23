mod tcp_server;
mod types;
mod udp_server;
mod utils;

use std::thread;

extern crate pnet;

struct Handler {
    list: Vec<types::Metadata>,
}

impl Handler {
    fn test(&mut self, meta: types::Metadata) {
        self.list.push(meta);
    }
}



fn main() -> Result<(), std::io::Error> {
    thread::spawn(|| {
        let mut meta_list = vec![types::Metadata {
            name: "fuji".to_string(),
            extension: "jpg".to_string(),
            name_extension: "fuji.jpg".to_string(),
            hash: "b0e490e762234567eaaaaaaaabc74fade854476fe692e320".to_string(),
            size: 124093,
        }];

        let mut hand = Handler {
            list: meta_list
        };

        // let test = fn _(meta: types::Metadata) {
        //     println!("THE FUCKING THING WORKS: {:?}", meta);
        // } 

        let mut tcp_server = tcp_server::TcpServer::new(&mut hand.list);
        tcp_server.listen(hand.test).unwrap();
    });

    let mdns = udp_server::UdpServer::new();
    mdns.listen().unwrap();

    Ok(())
}
