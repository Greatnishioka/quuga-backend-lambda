use std::sync::Arc;
use crate::domain::video::repository::video_repository::VideoRepository;

#[derive(Clone)]
pub struct AppState {
    pub repo: Arc<dyn VideoRepository>,
}