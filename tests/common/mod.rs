pub fn spawn() -> String {
    let listener = std::net::TcpListener::bind("localhost:0").expect("Failed to bind port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to listen to port");

    let _ = tokio::spawn(server);

    format!("http://localhost:{port}")
}
