#[cfg(test)]
mod client_manager_test {
    use server::client_manager::{ClientInfo, ClientManager};
    use std::{
        net::{IpAddr, Ipv4Addr, SocketAddr},
        sync::Arc,
        time::Duration,
    };
    use tokio::time::{Instant, sleep};

    // テスト: ClientManagerの初期化
    // 目的: タイムアウトが正しく設定され、クライアントテーブルが空であることを確認する
    #[test]
    fn client_manager_initializes_with_correct_timeout_and_empty_table() {
        // タイムアウト時間を10秒に設定してインスタンス化
        let timeout = Duration::from_secs(10);
        let manager = ClientManager::new(timeout);

        // タイムアウトが正しく設定されていることを検証
        assert_eq!(
            manager.timeout_duration, timeout,
            "タイムアウト時間が設定された値と一致するはず"
        );
        // 初期状態ではクライアントテーブルが空であることを検証
        assert!(
            manager.clients_table.is_empty(),
            "初期化直後はクライアントテーブルが空であるべき"
        );
    }

    // テスト: 単一クライアント追加
    // 目的: 単一のクライアントを追加して、カウントと格納内容を検証する
    #[test]
    fn test_add_single_client_into_client_manager() {
        let manager = ClientManager::new(Duration::from_secs(10));

        // クライアント情報を作成
        let client = ClientInfo {
            user_name: "alice".to_string(),
            socket_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
            last_message_time: Instant::now(),
        };

        // クライアントを追加
        manager.upsert_client(client.clone());

        // 追加後、アクティブクライアント数が1であることを検証
        assert_eq!(
            manager.active_client_count(),
            1,
            "クライアントを1つ追加したらカウントは1になるべき"
        );

        // テーブルに格納されたクライアント情報を取得して内容を検証
        let stored_client = manager.clients_table.get(&client.user_name).unwrap();
        assert_eq!(
            stored_client.value().user_name,
            client.user_name,
            "格納されたuser_nameが正しい"
        );
        assert_eq!(
            stored_client.value().socket_addr,
            client.socket_addr,
            "格納されたsocket_addrが正しい"
        );
    }

    // テスト: 複数クライアント同時追加
    // 目的: 複数のクライアントを追加できることを確認し、それぞれが管理されているか検証する
    #[test]
    fn test_add_multiple_clients_into_client_manager() {
        let manager = ClientManager::new(Duration::from_secs(10));

        // 最初のクライアントを作成
        let client1 = ClientInfo {
            user_name: "alice".to_string(),
            socket_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8081),
            last_message_time: Instant::now(),
        };
        // 2番目のクライアントを作成
        let client2 = ClientInfo {
            user_name: "bob".to_string(),
            socket_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8082),
            last_message_time: Instant::now(),
        };

        // それぞれのクライアントを追加
        manager.upsert_client(client1.clone());
        manager.upsert_client(client2.clone());

        // 追加後、アクティブクライアント数が2であることを検証
        assert_eq!(
            manager.active_client_count(),
            2,
            "クライアントを2つ追加したらカウントは2になるべき"
        );
        // 各ユーザー名がテーブルに存在することを検証
        assert!(
            manager.clients_table.contains_key(&client1.user_name),
            "alice がクライアントテーブルに存在するべき"
        );
        assert!(
            manager.clients_table.contains_key(&client2.user_name),
            "bob がクライアントテーブルに存在するべき"
        );
    }

    // テスト: 既存クライアントの最終活動時間更新
    // 目的: 既に存在するクライアントの last_message_time が更新されることを確認する
    #[test]
    fn test_update_existing_client_info() {
        let manager = ClientManager::new(Duration::from_secs(10));

        // 古い時間を last_message_time にセットしたクライアントを作成
        let initial_time = Instant::now() - Duration::from_secs(5); // 5秒前の時間を取得する
        let client = ClientInfo {
            user_name: "alice".to_string(),
            socket_addr: "127.0.0.1:8080".parse().unwrap(),
            last_message_time: initial_time,
        };

        // クライアントを追加
        manager.upsert_client(client.clone());

        // 更新前の時間を取得
        let before_update = manager
            .clients_table
            .get("alice")
            .unwrap()
            .value()
            .last_message_time;
        // update_client_activity を呼び出して更新
        manager.update_client_activity("alice").unwrap();
        // 更新後の時間を取得
        let after_update = manager
            .clients_table
            .get("alice")
            .unwrap()
            .value()
            .last_message_time;

        // 更新後の時間が更新前より後になっていることを検証
        assert!(
            after_update > before_update,
            "update_client_activity 呼び出し後に last_message_time が更新されるべき"
        );
    }

    // テスト: 存在しないクライアントの更新
    // 目的: クライアントが存在しない場合、エラーが返されることを確認する
    #[test]
    fn test_update_nonexistent_client() {
        let manager = ClientManager::new(Duration::from_secs(10));

        // 存在しないユーザー名で update_client_activity を呼び出す
        let result = manager.update_client_activity("alice");

        // Err が返され、メッセージに "not found" が含まれることを検証
        assert!(
            result.is_err(),
            "存在しないクライアントの更新では Err が返るべき"
        );
        assert!(
            result.err().unwrap().contains("not found"),
            "エラーメッセージに \"not found\" が含まれるべき"
        );
    }

    // テスト: 非アクティブクライアントのクリーンアップ
    // 目的: last_message_time がタイムアウトを超えたクライアントが削除されることを確認する
    #[test]
    fn test_cleanup_inactive_clients() {
        let manager = ClientManager::new(Duration::from_secs(10));

        // 過去の時間を last_message_time にセットしたクライアントを作成
        let inactive_time = Instant::now() - Duration::from_secs(20);
        let client = ClientInfo {
            user_name: "alice".to_string(),
            socket_addr: "127.0.0.1:8080".parse().unwrap(),
            last_message_time: inactive_time,
        };

        // クライアントを追加
        manager.upsert_client(client.clone());

        // クリーンアップ前にカウントが1であることを検証
        assert_eq!(
            manager.active_client_count(),
            1,
            "クリーンアップ前はアクティブクライアントが1であるべき"
        );

        // 非アクティブクライアントをクリーンアップ
        manager.cleanup_inactive_clients();

        // クリーンアップ後にカウントが0になり、テーブルに存在しないことを検証
        assert_eq!(
            manager.active_client_count(),
            0,
            "クライアントがタイムアウトを超えていればクリーンアップ後にカウントは0になるべき"
        );
        assert!(
            !manager.clients_table.contains_key("alice"),
            "クリーンアップ後に alice はテーブルに存在しないはず"
        );
    }

    // テスト: 同時アクセスによる複数クライアント追加
    // 目的: 複数のタスクから同時に upsert_client を呼び出しても安全に登録できることを確認する
    #[tokio::test]
    async fn test_concurrent_access() {
        let manager = Arc::new(ClientManager::new(Duration::from_secs(10)));
        let mut handles = Vec::new();

        // 5つの並行タスクでクライアントを追加
        for i in 0..5 {
            let mgr = Arc::clone(&manager);
            let client = ClientInfo {
                user_name: format!("user{}", i),
                socket_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8000 + i),
                last_message_time: Instant::now(),
            };
            handles.push(tokio::spawn(async move {
                mgr.upsert_client(client);
            }));
        }

        // 全タスクが終了するのを待つ
        for handle in handles {
            handle.await.unwrap();
        }

        // 5つのクライアントが登録されていることを検証
        assert_eq!(
            manager.active_client_count(),
            5,
            "同時実行後に登録されたクライアント数が5であるべき"
        );
    }

    // テスト: バックグラウンドクリーンアップタスクの動作
    // 目的: 指定したインターバルで自動的に非アクティブクライアントが削除されるかを確認する
    #[tokio::test]
    async fn test_background_cleanup_task() {
        // タイムアウトを1秒に設定
        let manager = Arc::new(ClientManager::new(Duration::from_secs(1)));

        // 過去の時間を last_message_time にセットしたクライアントを作成
        let client = ClientInfo {
            user_name: "alice".to_string(),
            socket_addr: "127.0.0.1:8080".parse().unwrap(),
            last_message_time: Instant::now() - Duration::from_secs(5),
        };

        // クライアントを追加（バックグラウンドタスクが動作中と想定）
        manager.upsert_client(client.clone());

        // バックグラウンドクリーンアップが実行されるまで待機
        sleep(Duration::from_secs(2)).await;

        // 2秒後にはクライアントがクリーンアップされ、カウントが0であることを検証
        assert_eq!(
            manager.active_client_count(),
            0,
            "バックグラウンドクリーンアップ後にアクティブクライアントは0であるべき"
        );
    }
}
