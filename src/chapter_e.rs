use crate::app_state::AppState;
use axum::extract::State;
use axum::response::{Html, Redirect};
use axum::routing::get;
use axum::{Form, Router};
use serde::Deserialize;
use tera::Context;

pub trait RegisterE {
    fn register_chapter_e(self) -> Router<AppState>;
}

impl RegisterE for Router<AppState> {
    fn register_chapter_e(self) -> Router<AppState> {
        self.route(
            "/chapter/e/birthday_wordle",
            get(chapter_e).post(chapter_e_validation),
        )
    }
}

async fn chapter_e(State(state): State<AppState>) -> Html<String> {
    let mut context = Context::new();

    let rendered = state
        .get_templates()
        .await
        .render("chapter_e.html", &context)
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
async fn chapter_e_validation(
    State(state): State<AppState>,
    Form(payload): Form<Payload>,
) -> Redirect {
    let current_date = chrono::offset::Local::now();
    let url = format!(
        "https://www.nytimes.com/svc/wordle/v2/{:04}-{:02}-{:02}.json",
        2025,
        04,
        08
    );
    let solution: WordleResponse = reqwest::get(url).await.unwrap().json().await.unwrap();

    if payload.value.to_lowercase() == solution.solution.to_lowercase() {
        Redirect::to("/chapter/f/timing")
    } else {
        Redirect::to("/chapter/e/birthday_wordle?failed=true")
    }
}
