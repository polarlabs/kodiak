use std::net::TcpListener;

#[actix_rt::test]
async fn health() {
    let address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health", address).as_str())
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    // Using a TcpListener allows to define the port to bind to, even to dynamic ports by using port 0
    // todo: error handling?
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();

    let server = kodiak_interface::run(listener).expect("Failed to start server.");
    tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
