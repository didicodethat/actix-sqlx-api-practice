pub mod database;
pub mod app;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use database::get_pool;
use dotenv::dotenv;
use std::env;
use app::user;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let address = env::var("HTTP_ADDRESS").unwrap_or(String::from("127.0.0.1"));
    let port = env::var("HTTP_PORT").unwrap_or(String::from("8080")).parse::<u16>().unwrap();

    println!("Attempting to run application on {}:{}", &address, &port);
    HttpServer::new(|| {
        App::new()
            .service(user::create)
            .service(index)
    })
        .bind((address, port))?
        .run()
        .await
}