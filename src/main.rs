use zero2prod::*;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    logs::init::<false>();

    let settings = configuration::Settings::new(None).expect("Failed to load configuration");
    tracing::info!(host = settings.routing.host, port = settings.routing.port, "Starting server");
    let listener = std::net::TcpListener::bind(settings.address())?;

    run(listener)?.await
}
