use std::sync::Arc;

use axum::{routing::get, Router};

use crate::domain::video::repository::video_repository::VideoRepository;
use crate::presentation::http::handler::get_video_handler::get_videos;

// tyeps
use crate::presentation::http::state::AppState;

pub fn build_router(repo: Arc<dyn VideoRepository>) -> Router {
    let state = AppState { repo };

    let v1_api = Router::new().route("/videos", get(get_videos));

    Router::new()
        .nest("/api", Router::new().nest("/v1", v1_api))
        .with_state(state)
}