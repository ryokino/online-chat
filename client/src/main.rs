use std::io;

use client::run_once;

#[tokio::main]
async fn main() -> io::Result<()> {
    run_once().await
}
