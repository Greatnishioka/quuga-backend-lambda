mod domain;
mod infrastructure;

use axum::{routing::get, Router};
use std::sync::Arc;
use axum::extract::Extension;
use serde_json;
use crate::infrastructure::in_memory_repo::InMemoryRepo;
use crate::domain::video::usecase::get_video::GetVideoUseCase;
use crate::domain::video::entity::video_entity::VideoId;

pub fn router() -> Router {
    // composition root: 実装をここで生成して注入する
    let repo = Arc::new(InMemoryRepo::new());
    Router::new()
        .route("/", get(root))
        .route("/demo_videos", get(demo_videos))
        .layer(Extension(repo))
}

pub async fn root() -> &'static str {
    "Hello Axum!"
}

async fn demo_videos(
    Extension(repo): Extension<Arc<InMemoryRepo>>
) -> String {
    let uc = GetVideoUseCase::new(repo.clone());
    let id = VideoId::new();
    match uc.execute(&id).await {
        Ok(v) => serde_json::to_string(&v).unwrap_or_default(),
        Err(_) => "[]".to_string(),
    }
}
