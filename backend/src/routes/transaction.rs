use actix_web::{get, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct Response {
    message: &'static str,
}

#[get("/transaction")]
pub async fn transaction() -> impl Responder {
    HttpResponse::Ok().json(Response { message: "Working" })
}
