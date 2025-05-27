#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct FormData {
    name: String,
    email: String,
}
pub(crate) async fn subscriptions(_form: actix_web::web::Form<FormData>) -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok()
}
