use crate::kits::scorecards::Kit;
use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;

pub async fn create() -> Router {
    let db = crate::io::db::Db::new().await;

    let scorecards_router = Kit::new(db).create_router();

    let api_router = Router::new().merge(scorecards_router);

    Router::new()
        .route("/status", get(|| async {}))
        .nest("/api", api_router)
        .layer(TraceLayer::new_for_http())
}
