use crate::utils::response_packet;
use bytes::Bytes;
use dns_message_parser::question::Question;
use dns_message_parser::Dns;
use dns_message_parser::DomainName;
use net2::UdpBuilder;
use std::net::{SocketAddr, UdpSocket};

pub struct UdpServer;

impl UdpServer {
    pub fn new() -> UdpServer {
        UdpServer
    }

    pub fn listen(&self) -> Result<(), std::io::Error> {
        let mut buf = [0 as u8; 512];
        let socket = UdpBuilder::new_v4()?
            .reuse_address(true)?
            .bind("127.0.0.1:5353")?;
        let (received, src_addr) = socket.recv_from(&mut buf).expect("Didnt received any data");
        let domain_name = self.get_domain_name(buf, received);

        self.send_response(socket, &domain_name, &src_addr)?;

        Ok(())
    }

    fn send_response(
        &self,
        socket: UdpSocket,
        domain_name: &DomainName,
        src_addr: &SocketAddr,
    ) -> Result<(), std::io::Error> {
        let same = domain_name.eq(&"_drive.local.");

        if same {
            socket.connect(src_addr).expect("Could not connect");
            let dns_packet = response_packet();

            socket
                .send(&dns_packet[..])
                .expect("Could not send message");
        }

        Ok(())
    }

    fn get_domain_name(&self, buf: [u8; 512], received: usize) -> DomainName {
        let packet = Bytes::copy_from_slice(&buf[..received]);
        let Dns { questions, .. } = Dns::decode(packet).unwrap();

        let Question { domain_name, .. } = questions
            .first()
            .expect("No question was asked in this packet");

        println!("{:#?}", domain_name);

        domain_name.clone()
    }
}
