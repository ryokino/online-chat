use std::io;
use tokio::net::UdpSocket;

/// Default ports & buffer sizes for the demo client.
pub const SERVER_PORT: u16 = 9001;
pub const CLIENT_PORT: u16 = 9050;
pub const BUFFER_SIZE: usize = 1024;

/// Ask the user for destination address and message payload,
/// then bind a UDP socket on `CLIENT_PORT`.
pub async fn set_up_client() -> io::Result<(UdpSocket, String, String)> {
    println!("\nType the server's address to connect to: ");
    let mut server_address = String::new();
    io::stdin()
        .read_line(&mut server_address)
        .expect("Failed to read line");
    let server_address = server_address.trim();

    println!("\nType message to send to server: ");
    let mut message = String::new();
    io::stdin()
        .read_line(&mut message)
        .expect("Failed to read line");
    let message = message.trim();

    let sock = UdpSocket::bind(format!("{}:{}", server_address, CLIENT_PORT)).await?;

    println!("\nClient is running on port {}", CLIENT_PORT);
    println!("\nServer is running on port {}", SERVER_PORT);

    Ok((sock, message.to_string(), server_address.to_string()))
}

/// Send the given message to the server.
pub async fn send_message(sock: &UdpSocket, message: &str, server_address: &str) -> io::Result<()> {
    println!("\nSending message to server…");
    let len = sock
        .send_to(
            message.as_bytes(),
            format!("{}:{}", server_address, SERVER_PORT),
        )
        .await?;
    println!("sent {len} bytes to {server_address}");
    Ok(())
}

/// Receive the echoed message from the server and print it.
pub async fn receive_message(sock: &UdpSocket) -> io::Result<()> {
    let mut buf = [0u8; BUFFER_SIZE];
    let (len, addr) = sock.recv_from(&mut buf).await?;
    println!("\n{len:?} bytes received from {addr:?}");

    let message = String::from_utf8_lossy(&buf[..len]);
    println!("\nReceived message: {message}");
    Ok(())
}

/// Convenience helper that performs one round‑trip and exits.
pub async fn run_once() -> io::Result<()> {
    let (sock, message, server_address) = set_up_client().await?;
    send_message(&sock, &message, &server_address).await?;
    receive_message(&sock).await?;
    println!("closing socket…");
    Ok(())
}
