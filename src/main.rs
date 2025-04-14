// Redeveloped with Actix Web
use actix_cors::Cors;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use mysql_async::{Pool, prelude::Queryable};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::Duration;
use crate::routes::configure_routes;

mod routes;
mod handlers;
mod models;
mod error;
mod services;




#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(configure_routes)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
