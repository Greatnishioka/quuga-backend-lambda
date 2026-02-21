// Video関係のLambda関数

mod shared;
mod presentation;
mod application;
mod domain;
mod infrastructure;

use crate::infrastructure::persistence::video::in_memory_repo::InMemoryRepo;
use crate::presentation::http::router::build_router;
use lambda_http::{run, Error};
use std::sync::Arc;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();
    info!("ここでAxum起動");
    run(build_router(Arc::new(InMemoryRepo::new()))).await
}