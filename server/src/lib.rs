use std::{io, sync::Arc};
use tokio::{net::UdpSocket, time::Instant};
use protocol::MessageProtocol;

pub mod client_manager;
use client_manager::{ClientInfo, ClientManager};

pub const SERVER_ADDRESS: &str = "0.0.0.0";
pub const SERVER_PORT: u16 = 9001;
pub const BUFFER_SIZE: usize = 1024;

/// Bind a UDP socket and prepare a fixed‑size buffer.
pub async fn set_up_server() -> io::Result<(UdpSocket, [u8; BUFFER_SIZE])> {
    let sock = UdpSocket::bind(format!("{}:{}", SERVER_ADDRESS, SERVER_PORT)).await?;
    let buf = [0; BUFFER_SIZE];
    println!("Server is running on port {}", SERVER_PORT);
    Ok((sock, buf))
}

/// Receive one datagram and echo it back.
pub async fn handle_client(sock: &UdpSocket, buf: &mut [u8; BUFFER_SIZE]) -> io::Result<()> {
    println!("\nWaiting for a message…");

    let (len, addr) = sock.recv_from(buf).await?;
    println!("\n{len:?} bytes received from {addr:?}");

    let message = String::from_utf8_lossy(&buf[..len]);
    println!("\nReceived message: {message}");

    if !message.is_empty() {
        let sent = sock.send_to(message.as_bytes(), addr).await?;
        println!("sent {sent} bytes to {addr:?}");
    }
    Ok(())
}

/// Receive one datagram, echo it back, and manage client information.
pub async fn handle_client_with_manager(
    sock: &UdpSocket,
    buf: &mut [u8; BUFFER_SIZE],
    client_manager: &Arc<ClientManager>,
) -> io::Result<()> {
    println!("\nWaiting for a message…");

    let (len, addr) = sock.recv_from(buf).await?;
    println!("\n{len:?} bytes received from {addr:?}");

    let message = String::from_utf8_lossy(&buf[..len]);
    println!("\nReceived message: {message}");

    if !message.is_empty() {
        // プロトコルに従ってメッセージを解析
        let user_name = match MessageProtocol::deserialize(&buf[..len]) {
            Ok(msg_protocol) => msg_protocol.user_name,
            Err(_) => {
                // プロトコル解析に失敗した場合は従来の方法でフォールバック
                message
                    .split_whitespace()
                    .next()
                    .unwrap_or("anonymous")
                    .to_string()
            }
        };

        // クライアント情報を作成・更新
        let client_info = ClientInfo {
            user_name: user_name.clone(),
            socket_addr: addr,
            last_message_time: Instant::now(),
        };

        // クライアントをテーブルに追加または更新
        client_manager.upsert_client(client_info);
        println!("Updated client info for user: {}", user_name);

        // メッセージをエコーバック
        let sent = sock.send_to(message.as_bytes(), addr).await?;
        println!("sent {sent} bytes to {addr:?}");
    }
    Ok(())
}

/// Production helper that runs forever.
pub async fn run_forever() -> io::Result<()> {
    let (sock, mut buf) = set_up_server().await?;
    loop {
        handle_client(&sock, &mut buf).await?;
        println!("--------------------------------");
    }
}
