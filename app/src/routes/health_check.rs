pub(crate) async fn health_check() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok()
}
