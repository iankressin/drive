use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io;
use std::str;

struct Metadata {
    name: String,
    extension: String,
    name_extension: String,
    size: String,
}

pub struct TcpServer;

impl TcpServer {
    pub fn new() -> TcpServer {
        TcpServer
    }

    pub fn listen(&self) -> Result<(), std::io::Error> {
        println!("TCP Listening...");

        let listener = TcpListener::bind("0.0.0.0:7878").unwrap();

        for stream in listener.incoming() {
            println!("Connection established!");
            let mut stream = stream.unwrap();

            self.handle_file(&mut stream);

            stream.write("Thanks".as_bytes())?;

            stream.flush().unwrap();
        }

        Ok(())
    }

    // TODO: Stream timeout
    fn handle_file(&self, stream: &mut TcpStream) {
        let meta_offset = 72;
        let mut buf = [0 as u8; 72];

        stream.read(&mut buf).unwrap();

        let metabuf = &buf[0..meta_offset];
        let metadata = self.get_metadata(&metabuf);
        let mut file = File::create(&metadata.name_extension).unwrap();

        io::copy(stream, &mut file).unwrap();
    }

    fn get_metadata(&self, metabuf: &[u8]) -> Metadata {
        let name = String::from_utf8_lossy(metabuf);
        let split = name.split(":");
        let data: Vec<&str> = split.collect();

        let name = data[0].to_string();
        let extension = data[1].to_string();
        let size = data[2].to_string();
        let name_extension = format!("{}.{}", name, extension);

        Metadata {
            name,
            extension,
            name_extension,
            size,
        }
    }
}
