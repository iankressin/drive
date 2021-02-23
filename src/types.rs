use serde::{Deserialize, Serialize};
use std::net::TcpStream;

pub type Callback = fn(Metadata);

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub name: String,
    pub extension: String,
    pub name_extension: String,
    pub size: u32,
    pub hash: String,
}

pub trait FileHandler {
    fn handle_metadata(meta: &mut Vec<Metadata>);

    fn handle_file(meta: Metadata, file: TcpStream);
}
