use dashmap::DashMap;
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio::time::Instant;

// クライアントの情報を保持する構造体
#[derive(Debug, Clone)]
pub struct ClientInfo {
    pub user_name: String,
    pub socket_addr: SocketAddr,
    pub last_message_time: Instant,
}

// クライアント情報を管理するマネージャー
pub struct ClientManager {
    // Dashboardを使用することで並列アクセス可能
    pub clients_table: Arc<DashMap<String, ClientInfo>>,
    pub timeout_duration: Duration,
}

impl ClientManager {
    pub fn new(timeout_duration: Duration) -> Self {
        Self {
            clients_table: Arc::new(DashMap::new()),
            timeout_duration,
        }
    }

    pub fn upsert_client(&self, client: ClientInfo) {}

    pub fn active_client_count(&self) -> usize {
        0
    }

    pub fn cleanup_inactive_clients(&self) {}

    pub fn update_client_activity(&self, user_name: &str) -> Result<(), String> {
        Ok(())
    }
}
