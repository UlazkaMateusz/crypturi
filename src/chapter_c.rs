use crate::app_state::AppState;
use axum::extract::State;
use axum::response::{Html, Redirect};
use axum::routing::get;
use axum::{Form, Router};
use serde::Deserialize;
use tera::Context;

pub trait RegisterC {
    fn register_chapter_c(self) -> Router<AppState>;
}

impl RegisterC for Router<AppState> {
    fn register_chapter_c(self) -> Router<AppState> {
        self.route("/chapter/c/a7f6f851-ae3a-4d47-8562-2b0e8fdb14a0", get(chapter_c).post(chapter_c_validation))
    }
}

async fn chapter_c(State(state): State<AppState>) -> Html<String> {
    let context = Context::new();

    let rendered = state
        .get_templates()
        .await
        .render("chapter_c.html", &context)
        .unwrap();

    Html(rendered)
}

#[derive(Deserialize)]
struct Payload {
    value: String,
}
async fn chapter_c_validation(
    State(state): State<AppState>,
    Form(payload): Form<Payload>,
) -> Redirect {
    if payload.value == "reja" {
        Redirect::to("/chapter/d/wordle")
    } else {
        Redirect::to("/chapter/c/a7f6f851-ae3a-4d47-8562-2b0e8fdb14a0")
    }
}
