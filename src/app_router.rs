use crate::kits::scorecards::Kit;
use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;

pub async fn create() -> Router {
    let db = crate::io::db::Db::new().await;

    let scorecards_kit = Kit::new(db);

    Router::new()
        .nest("/api", scorecards_kit.router())
        .route("/status", get(|| async {}))
        .layer(TraceLayer::new_for_http())
}
