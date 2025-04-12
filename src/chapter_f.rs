use crate::app_state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::{Json, Router};
use serde::Deserialize;
use tera::Context;

pub trait RegisterF {
    fn register_chapter_f(self) -> Router<AppState>;
}

impl RegisterF for Router<AppState> {
    fn register_chapter_f(self) -> Router<AppState> {
        self.route(
            "/chapter/f/timing",
            get(chapter_f).post(chapter_f_validation),
        )
    }
}

async fn chapter_f(State(state): State<AppState>) -> Html<String> {
    let mut context = Context::new();

    let rendered = state
        .get_templates()
        .await
        .render("chapter_f.html", &context)
        .unwrap();

    Html(rendered)
}

#[derive(Deserialize)]
struct Payload {
    value: String,
}

#[derive(Deserialize)]
struct WordleResponse {
    solution: String,
}
async fn chapter_f_validation(
    State(state): State<AppState>,
    Json(payload): Json<String>,
) -> impl IntoResponse {
    let solution = "080499";

    for i in 0..6 {
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        if payload.chars().nth(i) != Some(solution.chars().nth(i).unwrap()) {
            return (StatusCode::BAD_REQUEST, Json("Bad request"));
        }
    }

    if payload.len() != 6
    {
        return (StatusCode::BAD_REQUEST, Json("Bad request"));
    }

    (StatusCode::OK, Json("/finish/congratulations"))
}
