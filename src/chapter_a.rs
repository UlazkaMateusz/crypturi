use crate::app_state::AppState;
use axum::extract::{Path, State};
use axum::response::Html;
use axum::routing::get;
use axum::Router;
use tera::Context;

pub trait RegisterA {
    fn register_chapter_a(self) -> Router<AppState>;
}

impl RegisterA for Router<AppState> {
    fn register_chapter_a(self) -> Router<AppState> {
        self.route("/chapter/a/level/{level}", get(chapter_a))
    }
}

struct ChapterAContext {
    level: i32,
}

impl Into<Context> for ChapterAContext {
    fn into(self) -> Context {
        let mut context = Context::new();
        context.insert("level", &self.level);
        if self.level == 100 {
            context.insert(
                "special_button_text",
                "Let's go to chapter B");
            context.insert(
                "next_level_url",
                "/chapter/b/level/b4dff9a6-efd6-4556-b27e-1f64d7485b4e",
            );
        } else {
            context.insert(
                "next_level_url",
                &format!("/chapter/a/level/{}", self.level + 1),
            );
        }

        context
    }
}

async fn chapter_a(State(state): State<AppState>, Path(level): Path<i32>) -> Html<String> {
    let context = ChapterAContext { level };

    let rendered = state
        .get_templates()
        .await
        .render("chapter_a.html", &context.into())
        .unwrap();

    Html(rendered)
}
