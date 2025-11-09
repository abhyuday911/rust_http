use actix_web::{App, HttpResponse, HttpServer, Responder, Result, get, post, web};
use serde::Serialize;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[derive(Serialize)]
struct MyObj {
    name: String,
}

#[get("/a/{name}")]
async fn index(name: web::Path<String>) -> Result<impl Responder> {
    let obj = MyObj {
        name: name.to_string(),
    };
    Ok(web::Json(obj))
}
// _____________ multiple params _______________
#[derive(Serialize)]
struct MyObjMultiple {
    name: String,
    price: String,
    class: String,
}

#[get("/pet/{name}/{price}")]
async fn multiple_params(path: web::Path<(String, String)>) -> Result<impl Responder> {
    let (name, price) = path.into_inner();
    let obj = MyObjMultiple {
        name: name.to_string(),
        price: price.to_string(),
        class: String::from("godzilla"),
    };
    Ok(web::Json(obj))
}

// _____________ multiple params _______________

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(index)
            .service(multiple_params)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

