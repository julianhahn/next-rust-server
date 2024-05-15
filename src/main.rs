use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct ReleaseEvent {
    action: String,
    release: Release,
    repository: Repository,
    sender: User,
}

#[derive(Deserialize)]
struct Release {
   
}

#[derive(Deserialize)]
struct Repository {
   action: String,
}

#[derive(Deserialize)]
struct User {
    login: String,
    id: u64,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    #[serde(rename = "type")]
    user_type: String,
    site_admin: bool,
}

#[derive(Deserialize)]
struct Asset {
    url: String,
    id: u64,
    node_id: String,
    name: String,
    label: Option<String>,
    uploader: User,
    content_type: String,
    state: String,
    size: u64,
    download_count: u64,
    created_at: String,
    updated_at: String,
    browser_download_url: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/new-version")]
async fn new_version(event: web::Json<ReleaseEvent>) -> impl Responder {
    println!("Request body: {}", event.release.name);
    HttpResponse::Ok().body(format!("New version: {}", event.release.name))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(new_version))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
