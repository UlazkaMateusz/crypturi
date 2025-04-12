mod app_state;
mod chapter_a;
mod chapter_b;
mod welcome;
mod chapter_c;
mod chapter_d;
mod chapter_e;
mod chapter_f;
mod finish;

use crate::app_state::AppState;
use crate::chapter_a::RegisterA;
use crate::chapter_b::RegisterB;
use crate::chapter_c::RegisterC;
use crate::chapter_d::RegisterD;
use crate::chapter_e::RegisterE;
use crate::chapter_f::RegisterF;
use crate::finish::RegisterFinish;
use crate::welcome::RegisterWelcome;
use axum::routing::get_service;
use axum::Router;
use std::sync::Arc;
use tera::Tera;
use tokio::sync::Mutex;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    // Load templates
    let tera = Tera::new("templates/**/*").expect("Failed to initialize Tera");
    let state = AppState::new(Arc::new(Mutex::new(tera)));

    // Build router
    let app = Router::new()
        .register_welcome()
        .register_chapter_a()
        .register_chapter_b()
        .register_chapter_c()
        .register_chapter_d()
        .register_chapter_e()
        .register_chapter_f()
        .register_finish()
        .nest_service(
            "/res", // URL prefix where files will be served
            get_service(ServeDir::new("./public")))
        .with_state(state);

    // Run server
    let binds_str = "0.0.0.0:8040";
    let listener = tokio::net::TcpListener::bind(binds_str).await.unwrap();
    println!("Server running at http://{}", binds_str);
    axum::serve(listener, app).await.unwrap();
}
