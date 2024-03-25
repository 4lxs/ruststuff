use std::{io::Write, net::TcpStream};

fn handle_client(stream: TcpStream) {
    stream.write_fmt("hello");
}

fn main() -> std::io::Result<()> {
    let listener = std::net::TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        handle_client(stream?);
    }

    return Ok(());
}
