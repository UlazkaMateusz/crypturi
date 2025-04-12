use crate::app_state::AppState;
use axum::extract::State;
use axum::response::Html;
use axum::routing::get;
use axum::Router;
use tera::Context;

pub trait RegisterFinish {
    fn register_finish(self) -> Router<AppState>;
}

impl RegisterFinish for Router<AppState> {
    fn register_finish(self) -> Router<AppState> {
        self.route("/finish/congratulations", get(finish))
    }
}

async fn finish(State(state): State<AppState>) -> Html<String> {
    let context = Context::new();
    let rendered = state
        .get_templates()
        .await
        .render("finish.html", &context)
        .unwrap();

    Html(rendered)
}
