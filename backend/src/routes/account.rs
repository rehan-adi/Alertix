use serde::Serialize;
use actix_web::{ get, HttpResponse, Responder };

#[derive(Serialize)]
struct Response {
    message: &'static str
}

#[get("/account")]
pub async fn account() -> impl Responder {
    HttpResponse::Ok().json(Response {
        message: "Working"
    })
}