use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod routes;
mod handlers;
mod models;
mod services;
mod error;
mod database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 1. Load environment variables FIRST
    dotenv().expect("Failed to load .env file");
    println!("ENV VAR: {:?}", env::var("DATABASE_URL"));

    // 2. Create database pool
    let pool = database::create_pool().await
        .expect("Failed to create database pool");

    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    println!("Starting server on port {}", port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::configure_routes)
    })
        .bind(("127.0.0.1", port))?
        .run()
        .await
}