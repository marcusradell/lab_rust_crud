use crate::kits::scorecards::Kit;
use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;

pub async fn create() -> Router {
    let db = crate::io::db::Db::new().await;

    let scorecards_router = Kit::new(db).router();

    Router::new()
        .nest("/api", Router::new().merge(scorecards_router))
        .route("/status", get(|| async {}))
        .layer(TraceLayer::new_for_http())
}
