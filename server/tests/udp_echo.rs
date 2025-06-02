//! UDP echo integration test for the server library.
//!
//! 実際にソケットを開いて 1 往復だけ確認する。
//! 失敗するとタイムアウトでテストが落ちるので無限にハングらない。

use server::{BUFFER_SIZE, SERVER_PORT, handle_client, set_up_server};

use tokio::{
    task,
    time::{Duration, timeout},
};

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn echo_one_roundtrip() {
    // ❶ サーバをバックグラウンドで起動（1 パケット処理したら終了）
    let server = task::spawn(async {
        let (sock, mut buf) = set_up_server().await.unwrap();
        handle_client(&sock, &mut buf).await.unwrap();
    });

    // ❷ クライアントでメッセージ送信 → エコー受信
    let msg = "hello tokio";
    let echoed = timeout(Duration::from_secs(1), async {
        let client = tokio::net::UdpSocket::bind(("127.0.0.1", 0)).await.unwrap();
        client
            .send_to(msg.as_bytes(), ("127.0.0.1", SERVER_PORT))
            .await
            .unwrap();

        let mut buf = [0u8; BUFFER_SIZE];
        let (len, _) = client.recv_from(&mut buf).await.unwrap();
        String::from_utf8_lossy(&buf[..len]).into_owned()
    })
    .await
    .expect("client timed out");

    // ❸ Round‑trip が正しく行われたか検証
    assert_eq!(echoed, msg);

    // ❹ サーバタスクが正常終了したか確認
    server.await.unwrap();
}
