use crate::tcp_server;
use hana_types::Metadata;
use crate::udp_server;
use std::sync::mpsc::Sender;
use std::sync::{Arc, RwLock};
use std::thread;

pub struct DriveServer;

impl DriveServer {
    pub fn listen(
        lock: &Arc<RwLock<Vec<Metadata>>>,
        tx: Sender<Metadata>,
    ) -> Result<(), std::io::Error> {
        thread::spawn(|| {
            let mdns = udp_server::UdpServer::new();
            mdns.listen().unwrap();
        });

        let mut tcp_server = tcp_server::TcpServer::new(&lock);
        tcp_server.listen(tx).unwrap();

        Ok(())
    }
}
