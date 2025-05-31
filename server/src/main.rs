use std::io;
use tokio::net::UdpSocket;

const SERVER_ADDRESS: &str = "0.0.0.0";
const SERVER_PORT: u16 = 9001;
const BUFFER_SIZE: usize = 1024;

#[tokio::main]
async fn main() -> io::Result<()> {
    let (sock, mut buf) = set_up_server().await?;

    loop {
        handle_client(&sock, &mut buf).await?;
        println!("--------------------------------");
    }
}

async fn set_up_server() -> io::Result<(UdpSocket, [u8; BUFFER_SIZE])> {
    let sock = UdpSocket::bind(format!("{}:{}", SERVER_ADDRESS, SERVER_PORT)).await?;
    let buf = [0; BUFFER_SIZE];

    println!("Server is running on port {}", SERVER_PORT);

    Ok((sock, buf))
}

async fn handle_client(sock: &UdpSocket, buf: &mut [u8; BUFFER_SIZE]) -> io::Result<()> {
    println!("\nWaiting for a message...");

    let (len, addr) = sock.recv_from(buf).await?; // receive message from client
    println!("\n{:?} bytes received from {:?}", len, addr);

    let message = String::from_utf8_lossy(&buf[..len]);
    println!("\nReceived message: {}", message);

    if !message.is_empty() {
        let len = sock.send_to(message.as_bytes(), addr).await?;
        println!("sent {} bytes to {:?}", len, addr);
    }

    Ok(())
}
