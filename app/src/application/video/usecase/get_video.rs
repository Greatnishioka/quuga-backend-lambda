use std::sync::Arc;
use anyhow::{Result, anyhow};
use crate::domain::video::repository::video_repository::VideoRepository;
use crate::domain::video::entity::video_entity::{Video, VideoId};

pub struct GetVideoUseCase {
    // Arc<dyn VideoRepository>は、VideoRepositoryトレイトを実装した任意の構造体？あんま理解できてないかも
    repo: Arc<dyn VideoRepository>,
}

impl GetVideoUseCase {
    // 実際にGetVideoUseCaseを作成している関数。いつもLaravelでやってるcreateみたいな実装の仕方をRust流にやるとこう。
    pub fn new(repo: Arc<dyn VideoRepository>) -> Self { Self { repo } }

    // 実行部分。普段のLaravelではマジックメソッドを使用して、関数的に呼び出したら勝手に実行される関数を実装していたが、Rustではそれは過剰らしい。
    pub async fn execute(&self, id: &VideoId) -> Result<Video> {
        // Infra層との接続。
        match self.repo.find_by_id(id).await? {
            // データがあった場合の処理。Ok()で包んで返す。
            Some(v) => Ok(v),
            // データが空だった場合の処理。
            None => Err(anyhow!("video not found")),
        }
    }
}
