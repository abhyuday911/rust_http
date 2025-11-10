use crate::{AppState, User};
use actix_web::{
    HttpResponse, Responder,
    cookie::Cookie,
    web::{self, Data},
};
use bcrypt::{DEFAULT_COST, hash};
use serde_json::json;
use uuid::Uuid;

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("welcome to the / route")
}

pub async fn sign_up(state: Data<AppState>, user_data: web::Json<User>) -> impl Responder {
    let mut users = state.users.lock().unwrap();

    // check if user exists, if yes bhag yha se **** //
    // hash the user data
    // push it into the users hashmap. //
    // set cookie
    // send back response //

    if users.contains_key(&user_data.username) {
        return HttpResponse::Conflict().json(json!({"error": "User already exists"}));
    }

    let hashed_password = match hash(user_data.password.clone(), DEFAULT_COST) {
        Ok(hash) => hash,
        Err(_) => {
            return HttpResponse::Conflict().json(json!({"error" : "Hashing Failed, Kl aana"}));
        }
    };

    let new_user = User {
        username: user_data.username.clone(),
        name: user_data.name.clone(),
        password: hashed_password,
        age: user_data.age,
    };

    // cookie
    let user_session_id = Uuid::new_v4().to_string();
    {
        let mut session_ids = state.session_ids.lock().unwrap();
        session_ids.insert(user_session_id.clone(), user_data.username.clone());
    }

    let cookie = Cookie::build("session_id", user_session_id)
        .path("/")
        .secure(true)
        .http_only(false)
        .finish();

    users.insert(user_data.username.clone(), new_user);

    HttpResponse::Ok().cookie(cookie).json(json!(*users))
}

pub async fn sign_in() -> impl Responder {
    HttpResponse::Ok().body("sign_in route")
}
