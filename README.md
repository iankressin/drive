# Hana server

## Usage

```
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::sync::RwLock;
use drive_server::types::Metadata;
use drive_server::drive_server::DriveServer;

fn main() -> Result<(), std::io::Error> {
    // Source of truth
    let lock = Arc::new(RwLock::new(vec![types::Metadata {
        name: "fuji".to_string(),
        extension: "jpg".to_string(),
        name_extension: "fuji.jpg".to_string(),
        hash: "b0e490e762234567eabc74fade854476fe692e320".to_string(),
        size: 124093,
    }]));

    let c_lock = Arc::clone(&lock);

    let (tx, rx) = channel();

    thread::spawn(move || {
        for received in rx {
            let mut meta = lock.write().unwrap();
            println!("File received: {:?}", received);
            meta.push(received);
        }
    });

   DriveServer::listen(&c_lock, tx).unwrap();

```
