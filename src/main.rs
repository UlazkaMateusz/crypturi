use axum::{
    Router,
    extract::{Path, State},
    response::Html,
    routing::get,
};
use std::{net::SocketAddr, sync::Arc};
use tera::{Context, Tera};

#[derive(Clone)]
struct AppState {
    templates: Arc<Tera>,
}

#[tokio::main]
async fn main() {
    // Load templates
    let tera = Tera::new("templates/**/*").expect("Failed to initialize Tera");
    let state = AppState {
        templates: Arc::new(tera),
    };

    // Build router
    let app = Router::new()
        .route("/", get(index))
        .route("/chapter/a/level/{level}", get(chapter_a))
        .with_state(state);

    // Run server
    let binds_str = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(binds_str).await.unwrap();
    println!("Server running at http://{}", binds_str);
    axum::serve(listener, app).await.unwrap();
}

// Handler
async fn index(State(state): State<AppState>) -> Html<String> {
    let mut context = Context::new();
    context.insert("title", "Welcome Page");
    context.insert("heading", "Hello from Axum!");
    context.insert("content", "This HTML was rendered using Tera templates.");

    let rendered = state.templates.render("level_a.html", &context).unwrap();

    Html(rendered)
}

// Handler
async fn chapter_a(State(state): State<AppState>, Path(level): Path<i32>) -> Html<String> {
    let mut context = Context::new();
    context.insert("chapter", "A");
    context.insert("level", &level.to_string());
    context.insert("next_level_url", &format!("/chapter/a/level/{}", level + 1));

    let rendered = state.templates.render("level_a.html", &context).unwrap();

    Html(rendered)
}
