use actix_web::{HttpResponse, Responder, web};
use serde_json::json;

use crate::AppState;

pub async fn create_limit_order(state: web::Data<AppState>) -> impl Responder {
    let sender = state.trades_sender.clone();

    if let Err(_) = sender.send("ohio".to_string()).await {
        println!("receiver fropped");
        return HttpResponse::Conflict()
            .json(json!({"error": "something went wrong order not placed"}));
    };

    HttpResponse::Ok().body("hello world")
}
