mod database;
mod entity;

use std::env;
use std::io::Result;
use actix_web::{HttpServer, App, HttpResponse, get, post};
use database::{init_session, DATABASE_SESSION};
use serde::Serialize;
use sea_orm::{ActiveModelTrait, ActiveValue::Set};

use entity::samples::{Entity as Samples, ActiveModel as SamplesModel};
use sea_orm::EntityTrait;

#[derive(Serialize, Debug)]
#[allow(non_snake_case)]
struct SamplesJson {
    id: i32,
    name: String,
}

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("It works!")
}

#[get("/ok")]
async fn ok() -> HttpResponse {
    HttpResponse::Ok().body("OK")
}

#[get("/api/v1/samples")]
async fn get_samples() -> HttpResponse {
    let db = DATABASE_SESSION.get().expect("Database session not initialized");
    let samples = Samples::find().all(&db.connection).await.unwrap()
        .into_iter().map(|s| SamplesJson { id: s.id, name: s.name }).collect::<Vec<SamplesJson>>();
    let json = serde_json::to_string(&samples).unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .body(json)
}

#[post("/api/v1/samples")]
async fn post_samples() -> HttpResponse {
    println!("POST /api/v1/samples");
    let db = DATABASE_SESSION.get().expect("Database session not initialized");
    let name = "test sample".to_string();
    let sample = SamplesModel {
        name: Set(name.clone()),
        ..Default::default()
    };
    match sample.save(&db.connection).await {
        Ok(s) => {
            let id = s.id.clone().take().unwrap();
            let name = s.name.clone().take().unwrap();
            let json = serde_json::to_string(&SamplesJson { id: id, name: name }).unwrap();
            HttpResponse::Ok()
                .content_type("application/json")
                .body(json)
        }
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to save sample: {:?}", err))
        }
    }
}

#[actix_web::main]
async fn main() -> Result<()> {
    let ip_address = env::var("IP_ADDRESS").unwrap_or_else(|_| "127.0.0.1".to_string());
    let p = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let port = u16::from_str_radix(&p, 10).unwrap();
    init_session().await.unwrap();

    println!("Starting server on port {}", port);

    HttpServer::new(|| {
        App::new()
            .service(ok)
            .service(index)
            .service(get_samples)
            .service(post_samples)
    })
    .bind((ip_address, port))?
    .run()
    .await
}
