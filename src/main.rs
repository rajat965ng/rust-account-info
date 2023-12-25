use actix_web::{App, HttpServer};
use actix_web::web::Data;
use dotenv::dotenv;
use mongodb::Client;

mod controller;
mod model;
mod service;


#[actix_web::main]
async fn main() -> std::io::Result<()>{
    dotenv().ok();
    let client = Client::with_uri_str("mongodb://localhost:27017").await.unwrap();
    let database = client.database("account");
    HttpServer::new(move || {
        App::new()
            .configure(controller::config)
            .app_data(Data::new(database.clone()))
    }).bind(("0.0.0.0", 8080))?.run().await
}