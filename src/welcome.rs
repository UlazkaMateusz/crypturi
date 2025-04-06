use crate::app_state::AppState;
use axum::extract::State;
use axum::response::Html;
use axum::routing::get;
use axum::Router;
use tera::Context;

pub trait RegisterWelcome {
    fn register_welcome(self) -> Router<AppState>;
}

impl RegisterWelcome for Router<AppState> {
    fn register_welcome(self) -> Router<AppState> {
        self.route("/", get(welcome))
    }
}

async fn welcome(State(state): State<AppState>) -> Html<String> {
    let context = Context::new();
    let rendered = state
        .get_templates()
        .await
        .render("welcome.html", &context)
        .unwrap();

    Html(rendered)
}
