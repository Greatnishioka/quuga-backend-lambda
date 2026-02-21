// Video関係のLambda関数

mod application;
mod domain;
mod infrastructure;

use crate::domain::video::entity::video_entity::VideoId;
use crate::application::video::usecase::get_video::GetVideoUseCase;
use crate::infrastructure::persistence::video::in_memory_repo::InMemoryRepo;
use axum::{extract::Extension, routing::get, Router};
use lambda_http::{run, Error};
use serde_json;
use std::sync::Arc;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();
    info!("ここでAxum起動");
    run(router()).await
}

// ルーター定義
fn router() -> Router {
    // composition root: 実装をここで生成して注入する
    let repo = Arc::new(InMemoryRepo::new());

    let v1_api = Router::new()
        .route("/videos", get(get_videos))
        // ここもうちょっとマシにしたい
        .layer(Extension(repo));

    return Router::new().nest("/api", Router::new().nest("/v1", v1_api));
}

async fn get_videos(Extension(repo): Extension<Arc<InMemoryRepo>>) -> String {
    let uc = GetVideoUseCase::new(repo.clone());
    let id = VideoId::new();
    match uc.execute(&id).await {
        Ok(v) => serde_json::to_string(&v).unwrap_or_default(),
        Err(_) => "[]".to_string(),
    }
}
