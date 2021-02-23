// TODO: Thread pool
// TODO: Check hashes with Merkle tree
// TODO: Extract meta handling to a new file
// TODO: Compare if the file is being waited
use crate::types::{Metadata, Callback};
use std::collections::HashMap;
use std::fs::{File};
use std::io;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::str;
use std::sync::mpsc::{self, channel};
use std::thread;

pub struct TcpServer<'a> {
    metadata: &'a mut Vec<Metadata>,
    waiting_list: HashMap<String, String>,
}

impl<'a> TcpServer<'a> {
    pub fn new(metadata: &'a mut Vec<Metadata>) -> TcpServer<'a> {
        TcpServer {
            metadata,
            waiting_list: HashMap::new(),
        }
    }

    pub fn listen(&mut self, cb: Callback) -> Result<(), std::io::Error> {
        println!("TCP Listening...");

        let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
        let (tx, rx) = channel();

        thread::spawn(move || {
            for received in rx {
                cb(received);
            }
        });

        // The first part of the handshake is to receive the
        // metadata file which contains the files that the client
        // is trying to send and decide which files the server
        // want to receive
        for stream in listener.incoming() {
            // A thread that is responsible for writing to meta file
            // Which operation the client wants to execute
            let mut op = [0 as u8; 1];
            let mut stream = stream.unwrap();
            stream.read(&mut op).unwrap();

            match op[0] {
                0u8 => {
                    self.handle_metadata(&mut stream);
                }
                1u8 => {
                    let tx_pipe = mpsc::Sender::clone(&tx);

                    thread::spawn(move || {
                        let meta = TcpServer::handle_file(&mut stream);

                        match tx_pipe.send(meta) {
                            Ok(_) => println!("Received"),
                            Err(err) => println!("Erro: {}", err),
                        }
                    });
                }
                _ => println!("No op setted in the packet")
            }
        }

        Ok(())
    }

    // Tells the client which file the server wants to receive
    // and store their hashes locally
    fn handle_metadata(&mut self, stream: &mut TcpStream) {
        let mut buf = [0 as u8; 1024];
        stream.read(&mut buf).unwrap();

        // This could be a problem if buffer has a 0 in the middle of it
        // TODO: Find a better solution
        let eos = buf.iter().position(|&r| r == 0).unwrap();
        // println!("End of stream >>>>>>>> {:?}, {}", buf, eos);
        let json = String::from_utf8_lossy(&buf[..eos]);
        println!("FILES BEING TRANSFERED: {}", json);

        let incoming_metadata: Vec<Metadata> = serde_json::from_str(&json).unwrap();

        let requested_files = self.pick_files(&incoming_metadata);

        stream.write(requested_files.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    fn pick_files(&mut self, incoming_metadata: &Vec<Metadata>) -> String {
        let mut requested_files: Vec<&Metadata> = Vec::new();

        match self.metadata.len() {
            0 => {
                println!("The metada file is empty");
                serde_json::to_string(&incoming_metadata).unwrap()
            }
            _ => {
                // TODO: Refactor pls
                for incoming_file in incoming_metadata {
                    let mut found = false;

                    for meta in self.metadata.iter() {
                        println!("Files to pick: {:?}, {:?}", incoming_file.hash, meta.hash);
                        if incoming_file.hash == meta.hash {
                            found = true;
                        }
                    }

                    if !found {
                        self.waiting_list.insert(
                            incoming_file.hash.to_owned(),
                            incoming_file.name_extension.to_owned(),
                        );
                        requested_files.push(incoming_file);
                    }
                }
                serde_json::to_string(&requested_files).unwrap()
            }
        }
    }

    // TODO: Stream timeout
    // TODO: Check if the server is waiting for the file
    fn handle_file(stream: &mut TcpStream) -> Metadata {
        let meta_offset = 72;
        let mut buf = [0 as u8; 72];

        stream.read(&mut buf).unwrap();

        let metabuf = &buf[0..meta_offset];
        let metadata = TcpServer::get_metadata(&metabuf);

        let mut file = File::create(&metadata.name_extension).unwrap();

        println!("Writing the file to the disk");
        io::copy(stream, &mut file).unwrap();

        metadata
    }

    fn push_metadata(&mut self, meta: Metadata) {
        self.metadata.push(meta);
    }

    fn read_metadata_file(&mut self) -> &mut Vec<Metadata> {
        // let json = fs::read_to_string("./.drive/.meta.json")?;
        // let current_metadata: Vec<Metadata> = serde_json::from_str(&json).unwrap();

        self.metadata
    }

    fn get_metadata(metabuf: &[u8]) -> Metadata {
        let eoh = metabuf.iter().position(|&r| r == 0).unwrap();

        let name = String::from_utf8_lossy(&metabuf[..eoh]);

        let split = name.split(":");
        let data: Vec<&str> = split.collect();

        let hash = data[0].to_string();
        let name = data[1].to_string();
        let extension = data[2].to_string();
        let name_extension = format!("{}.{}", name, extension);

        Metadata {
            name,
            extension,
            name_extension,
            size: 0,
            hash,
        }
    }

    fn check_hash() {}
}
