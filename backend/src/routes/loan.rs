use actix_web::{get, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct Response {
    message: &'static str,
}

#[get("/loan")]
pub async fn loan() -> impl Responder {
    HttpResponse::Ok().json(Response { message: "Working" })
}
