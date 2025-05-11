use zero2prod::*;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = std::net::TcpListener::bind("localhost:8000")?;
    run(listener)?.await
}
