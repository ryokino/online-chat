use std::{io, sync::Arc, time::Duration};

use server::{client_manager::ClientManager, handle_client_with_manager, set_up_server};

#[tokio::main]
async fn main() -> io::Result<()> {
    let (sock, mut buf) = set_up_server().await?;

    // クライアント管理機能を初期化（30秒のタイムアウト、バックグラウンドクリーンアップ有効）
    let client_manager = Arc::new(ClientManager::new_with_background_cleanup(
        Duration::from_secs(30),
    ));

    println!("Client manager initialized with 30s timeout and background cleanup");

    loop {
        handle_client_with_manager(&sock, &mut buf, &client_manager).await?;
        println!("Active clients: {}", client_manager.active_client_count());
        println!("--------------------------------");
    }
}
