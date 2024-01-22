use crate::kits::scorecards::Kit;
use axum::Router;

pub async fn create() -> Router {
    let db = crate::io::db::Db::new().await;

    let scorecards_kit = Kit::new(db);

    Router::new().nest(
        &format!("/{}", module_path!().split("::").last().unwrap()),
        scorecards_kit.router(),
    )
}
