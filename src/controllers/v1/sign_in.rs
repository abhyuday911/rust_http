use actix_web::{HttpResponse, Responder};

pub async fn sign_in() -> impl Responder {
    HttpResponse::Ok().body("sign_in route")
}