use crate::tcp_server;
use crate::udp_server;
use hana_types::Metadata;
use std::sync::mpsc::Sender;
use std::sync::{Arc, RwLock};
use std::thread;

pub struct HanaServer;

impl HanaServer {
    pub fn listen(
        lock: &Arc<RwLock<Vec<Metadata>>>,
        tx: Sender<Metadata>,
        path: &str,
        keep_alive: bool,
    ) -> Result<(), std::io::Error> {
        let mut tcp_server = tcp_server::TcpServer::new(path, &lock);
        tcp_server.listen(tx, &keep_alive).unwrap();
        
        thread::spawn(move || {
            let mdns = udp_server::UdpServer::new();
            mdns.listen(&keep_alive).unwrap();
        });


        Ok(())
    }
}
