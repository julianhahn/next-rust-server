use actix_web::{http::StatusCode, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct ReleaseEvent {
    release: Release,
}

#[derive(Deserialize)]
struct Release {
    created_at: String,
    id: u32,
    name: String,
    tag_name: String,
}

#[post("/new-version")]
async fn new_version(event: web::Json<ReleaseEvent>) -> (impl Responder, StatusCode) {
    println!("name: {}", event.release.name);
    println!("tag_name: {}", event.release.tag_name);
    println!("created_at: {}", event.release.created_at);
    println!("id: {}", event.release.id);
    (
        HttpResponse::Ok().body(format!("New version: {}", event.release.tag_name)),
        StatusCode::OK,
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(new_version))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
