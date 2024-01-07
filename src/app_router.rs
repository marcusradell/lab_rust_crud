use crate::kits::scorecards::ScorecardsKit;
use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;

pub async fn create() -> Router {
    let scorecards_router = ScorecardsKit::new().await.create_router();
    let api_router = Router::new().nest("/scorecards", scorecards_router);

    Router::new()
        .route("/status", get(|| async {}))
        .nest("/api", api_router)
        .layer(TraceLayer::new_for_http())
}
