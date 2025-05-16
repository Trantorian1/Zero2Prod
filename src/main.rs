use zero2prod::*;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let settings = configuration::Settings::new(None);
    let listener = std::net::TcpListener::bind(settings.address())?;
    run(listener)?.await
}
