#![allow(unused)]

mod commands;
mod errors;
mod memtable;
mod store;

use bincode;
use commands::Commands;
use errors::Errors;
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

impl std::fmt::Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueType::String(s) => write!(f, "\"{s}\""),
            ValueType::Int(i) => write!(f, "{i}"),
        }
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let store = Arc::new(RwLock::new(Store::new()));

    let repl_handle = tokio::spawn(repl(Arc::clone(&store)));

    let server_handle = tokio::spawn(async move {
        let listener = TcpListener::bind("127.0.0.1:8080")
            .await
            .expect("Failed to bind to port 3000");

        loop {
            let (socket, _) = listener
                .accept()
                .await
                .expect("Failed to accept connection");

            tokio::spawn(handle_connection(socket, Arc::clone(&store)));
        }
    });

    tokio::select! {
        _ = repl_handle => {
            println!("Exiting");
        }
        _ = server_handle => {

        }
    }

    Ok(())
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
                Commands::Get => match check_num_params(Commands::Get, &tokens) {
                    Ok(()) => {
                        let store_read = store.read().unwrap();
                        match store_read.get(tokens[1].to_string()) {
                            Some(v) => println!("{}", v),
                            None => println!("Key not found"),
                        };
                    }
                    Err(()) => eprintln!("{:?}", Errors::IncorrectArgCount),
                },
                Commands::Put => match check_num_params(Commands::Put, &tokens) {
                    Ok(()) => {
                        let mut store_write = store.write().unwrap();
                        store_write.put(
                            tokens[1].to_string(),
                            ValueType::String(tokens[2].to_string()),
                        );
                    }
                    Err(()) => eprintln!("{:?}", Errors::IncorrectArgCount),
                },
                Commands::Exit => {
                    break 'outer;
                }
            },
            Err(e) => eprintln!("{:?}", e),
        }
    }
}

fn check_num_params(c_type: Commands, tokens: &Vec<&str>) -> Result<(), ()> {
    if tokens.len() != c_type.num_args() + 1 {
        Err(())
    } else {
        Ok(())
    }
}
