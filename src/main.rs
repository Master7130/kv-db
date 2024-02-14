mod commands;
mod errors;
mod memtable;
mod store;

use bincode;
use commands::Commands;
use serde::Deserialize;
use std::{
    io::{self, BufRead, Write},
    sync::{Arc, RwLock},
};
use store::Store;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[derive(Debug, Deserialize)]
struct Message<'a>(&'a str);

impl Default for Message<'_> {
    fn default() -> Self {
        Message("")
    }
}

#[derive(Debug, Clone)]
enum ValueType {
    String(String),
    Int(i64),
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let store = Arc::new(RwLock::new(Store::new()));

    let res = tokio::spawn(repl(Arc::clone(&store)));

    let listener = TcpListener::bind("0.0.0.1:3000").await?;

    loop {
        let (socket, _) = listener.accept().await?;

        tokio::spawn(handle_connection(socket, Arc::clone(&store)));
    }
}

async fn handle_connection(mut socket: TcpStream, store: Arc<RwLock<Store>>) {
    let mut buf = [0; 1024];

    loop {
        let nbytes = {
            let result = socket.read(&mut buf).await;
            match result {
                Ok(0) => todo!(),
                Ok(n) => n,
                Err(e) => {
                    eprintln!("Error reading from socket: {:?}", e);
                    continue;
                }
            }
        };

        let msg = match bincode::deserialize::<Message>(&buf[..nbytes]) {
            Ok(msg) => msg,
            Err(_) => {
                // TODO: Handle deserialization error
                continue;
            }
        };

        // TODO: Process the message
    }
}

async fn repl(store: Arc<RwLock<Store>>) {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let stdout = io::stdout();
    let mut writer = stdout.lock();

    'outer: loop {
        write!(writer, "> ").expect("Failed to write");
        writer.flush().expect("Failed to flush output");

        let mut line = String::new();
        let bytes = reader.read_line(&mut line).expect("Failed to read line");

        if bytes == 0 {
            continue;
        }

        let line = line.trim();

        let tokens = line.split(" ");
        let tokens = tokens.collect::<Vec<&str>>();

        match &tokens[0].parse::<Commands>() {
            Ok(command) => match command {
                Commands::Get => {
                    let store_read = store.read().unwrap();
                    match store_read.get(tokens[1].to_string()) {
                        Some(v) => println!("{:?}", v),
                        None => println!("Key not found"),
                    };
                }
                Commands::Put => {
                    let mut store_write = store.write().unwrap();
                    store_write.put(tokens[1].to_string(), ValueType::String(tokens[2].to_string()));
                },
                // Commands::Exit => {
                //     break 'outer;
                // }
            },
            Err(e) => eprintln!("{:?}", e),
        }
    }
}
