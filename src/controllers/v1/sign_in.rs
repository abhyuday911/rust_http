use actix_web::{
    HttpResponse, Responder,
    web::{self, Data},
};
use bcrypt::verify;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::AppState;

#[derive(Serialize, Deserialize, Clone)]
pub struct Signup {
    username: String,
    password: String,
}

pub async fn sign_in(state: Data<AppState>, user_data: web::Json<Signup>) -> impl Responder {
    let users = state.users.lock().unwrap();

    // if !users.contains_key(&user_data.username) {
    //     return HttpResponse::Conflict().json(json!({"error": "user not found"}))
    // }

    let user = match users.get(&user_data.username) {
        Some(u) => u,
        None => return HttpResponse::Conflict().json(json!({"error" : "Invalid Credentials"})),
    };

    // let verify = verify(&user_data.password, &user.password);

    match verify(&user_data.password, &user.password) {
        Ok(v) => v,
        Err(_) => {
            return HttpResponse::Ok().json(json!({"error" : "Wrong Password"}));
        }
    };

    HttpResponse::Ok().json(json!(user))
}
