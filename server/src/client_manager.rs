use dashmap::DashMap;
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio::time::{Instant, interval};

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

    pub fn new_with_background_cleanup(timeout_duration: Duration) -> Self {
        let manager = Self {
            clients_table: Arc::new(DashMap::new()),
            timeout_duration,
        };

        // バックグラウンドクリーンアップタスクを開始
        let manager_clone = Arc::clone(&manager.clients_table);
        let timeout_duration_clone = timeout_duration;

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(1));
            loop {
                interval.tick().await;
                let now = Instant::now();
                manager_clone.retain(|_, client| {
                    now.duration_since(client.last_message_time) < timeout_duration_clone
                });
            }
        });

        manager
    }

    pub fn upsert_client(&self, client: ClientInfo) {
        self.clients_table.insert(client.user_name.clone(), client);
    }

    pub fn active_client_count(&self) -> usize {
        self.clients_table.len()
    }

    pub fn cleanup_inactive_clients(&self) {
        let now = Instant::now();
        self.clients_table.retain(|_, client| {
            now.duration_since(client.last_message_time) < self.timeout_duration
        });
    }

    pub fn update_client_activity(&self, user_name: &str) -> Result<(), String> {
        if let Some(mut client) = self.clients_table.get_mut(user_name) {
            client.last_message_time = Instant::now();
            Ok(())
        } else {
            Err(format!("Client '{}' not found", user_name))
        }
    }
}
