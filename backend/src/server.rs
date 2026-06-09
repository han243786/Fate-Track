use crate::app::{App, parse_and_handle};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

pub struct TcpServer {
    listener: TcpListener,
}

impl TcpServer {
    pub fn bind(addr: &str) -> std::io::Result<Self> {
        Ok(Self {
            listener: TcpListener::bind(addr)?,
        })
    }

    pub fn serve(self, app: App) -> std::io::Result<()> {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => handle_stream(stream, &app),
                Err(error) => eprintln!("connection error: {error}"),
            }
        }

        Ok(())
    }
}

fn handle_stream(mut stream: TcpStream, app: &App) {
    let mut buffer = [0_u8; 4096];
    let bytes_read = match stream.read(&mut buffer) {
        Ok(size) => size,
        Err(error) => {
            eprintln!("request read error: {error}");
            return;
        }
    };

    if bytes_read == 0 {
        return;
    }

    let response = parse_and_handle(app, &buffer[..bytes_read]);

    if let Err(error) = stream.write_all(response.to_http().as_bytes()) {
        eprintln!("response write error: {error}");
    }
}
