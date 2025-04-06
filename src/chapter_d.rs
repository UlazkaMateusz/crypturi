use crate::app_state::AppState;
use axum::extract::State;
use axum::response::{Html, Redirect};
use axum::routing::get;
use axum::{Form, Router};
use chrono::Datelike;
use serde::Deserialize;
use tera::Context;

pub trait RegisterD {
    fn register_chapter_d(self) -> Router<AppState>;
}

impl RegisterD for Router<AppState> {
    fn register_chapter_d(self) -> Router<AppState> {
        self.route(
            "/chapter/d/wordle",
            get(chapter_d).post(chapter_d_validation),
        )
    }
}

async fn chapter_d(State(state): State<AppState>) -> Html<String> {
    let mut context = Context::new();

    let rendered = state
        .get_templates()
        .await
        .render("chapter_d.html", &context)
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
async fn chapter_d_validation(
    State(state): State<AppState>,
    Form(payload): Form<Payload>,
) -> Redirect {
    let current_date = chrono::offset::Local::now();
    let url = format!(
        "https://www.nytimes.com/svc/wordle/v2/{:04}-{:02}-{:02}.json",
        current_date.year(),
        current_date.month(),
        current_date.day()
    );
    let solution: WordleResponse = reqwest::get(url).await.unwrap().json().await.unwrap();

    if payload.value == solution.solution {
        Redirect::to("/chapter/e/TODO")
    } else {
        Redirect::to("/chapter/d/wordle?failed=true")
    }
}
