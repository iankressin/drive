use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str;
// use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
struct Metadata {
    name: String,
    extension: String,
    name_extension: String,
    size: u32,
    hash: String,
}

pub struct TcpServer;

impl TcpServer {
    pub fn new() -> TcpServer {
        TcpServer
    }

    pub fn listen(&self) -> Result<(), std::io::Error> {
        println!("TCP Listening...");

        let listener = TcpListener::bind("0.0.0.0:7878").unwrap();

        // The first part of the handshake is to receive the
        // metadata file which contains the files that the client
        // is trying to send and decide which files the server
        // want to receive
        for stream in listener.incoming() {
            println!("Connection established!");
            // Which operation the client wants to execute
            let mut op = [0 as u8; 1];
            let mut stream = stream.unwrap();
            stream.read(&mut op).unwrap();

            if op[0] == 0 {
                // Tells the client which file the server wants to receive
                // and store their hashes locally
                self.handle_metadata(&mut stream);
            }

            if op[0] == 1 {
                println!("File")
            }

            // Keep receiving the files untill the client closes stream
            // TODO: Handle stream of multiple files
            // while !eof {
            //     handle_file
            // }

            // println!("{:?}", &buf[..]);
        }

        Ok(())
    }

    fn handle_metadata(&self, stream: &mut TcpStream) {
        let mut buf = [0 as u8; 1024];
        stream.read(&mut buf).unwrap();

        // This could be a problem if buffer has a 0 in the middle of it
        // TODO: Find a better solution
        let eos = buf.iter().position(|&r| r == 0).unwrap();
        let json = String::from_utf8_lossy(&buf[..eos]);
        let incoming_metadata: Vec<Metadata> = serde_json::from_str(&json).unwrap();

        for meta in &incoming_metadata {
            println!("{:#?}", meta);
        }

        let requested_files = self.pick_files(&incoming_metadata);

        stream.write(requested_files.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    fn pick_files(&self, incoming_metadata: &Vec<Metadata>) -> String {
        let json_metadata =
            fs::read_to_string("./.drive/.meta.json")
            .expect("Couldn't open the file");

        let current_metadata: Vec<Metadata> = 
            serde_json::from_str(&json_metadata)
            .unwrap();

        let mut requested_files: Vec<&Metadata> = Vec::new();

        // TODO: Find a better algorithm or datascructure to
        // find the missing files
        for meta in incoming_metadata {
            for data in &current_metadata {
                if meta.hash != data.hash {
                    requested_files.push(meta);
                }
            }
        }

        serde_json::to_string(&requested_files).unwrap()
    }

    fn check_hash() {}

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
            size: 0,
            hash: String::from(""),
        }
    }
}
