pub mod configuration;
pub mod logs;

pub(crate) mod serialize;

mod routes;

pub fn run(listener: std::net::TcpListener) -> Result<actix_web::dev::Server, std::io::Error> {
    let server = actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .route("/health_check", actix_web::web::get().to(routes::health_check))
            .route("/subscriptions", actix_web::web::post().to(routes::subscriptions))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
