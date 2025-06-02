use std::io;

use server::{handle_client, set_up_server};

#[tokio::main]
async fn main() -> io::Result<()> {
    let (sock, mut buf) = set_up_server().await?;

    loop {
        handle_client(&sock, &mut buf).await?;
        println!("--------------------------------");
    }
}
