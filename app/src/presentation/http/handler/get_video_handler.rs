// 汎用
use axum::{extract::State};
use serde_json;

use crate::application::video::usecase::get_video::GetVideoUseCase;
use crate::domain::video::entity::video_entity::VideoId;

// tyeps
use crate::presentation::http::state::AppState;

pub async fn get_videos(State(state): State<AppState>) -> String {
    let uc = GetVideoUseCase::new(state.repo.clone());
    let id = VideoId::new();

    match uc.execute(&id).await {
        Ok(v) => serde_json::to_string(&v).unwrap_or_default(),
        Err(_) => "[]".to_string(),
    }
}