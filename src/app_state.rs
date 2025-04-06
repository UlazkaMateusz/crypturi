use std::sync::Arc;
use tera::Tera;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    templates: Arc<Mutex<Tera>>,
}

impl AppState {
    pub fn new(templates: Arc<Mutex<Tera>>) -> AppState {
        AppState { templates }
    }

    pub async fn get_templates(&self) -> Tera {
        let mut templates = self.templates.lock().await;
        templates.full_reload().unwrap();
        templates.clone()
    }
}
