mod app_state;
mod chapter_a;
mod chapter_b;
mod welcome;
mod chapter_c;
mod chapter_d;

use crate::app_state::AppState;
use crate::chapter_a::RegisterA;
use crate::chapter_b::RegisterB;
use crate::chapter_c::RegisterC;
use crate::chapter_d::RegisterD;
use crate::welcome::RegisterWelcome;
use axum::Router;
use std::sync::Arc;
use tera::Tera;
use tokio::sync::Mutex;

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
        .with_state(state);

    // Run server
    let binds_str = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(binds_str).await.unwrap();
    println!("Server running at http://{}", binds_str);
    axum::serve(listener, app).await.unwrap();
}
