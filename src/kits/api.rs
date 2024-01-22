use crate::{io::db::Db, kits::scorecards::Kit};
use axum::Router;

pub async fn router(db: Db) -> Router {
    let scorecards_kit = Kit::new(db);

    Router::new().nest(
        &format!("/{}", module_path!().split("::").last().unwrap()),
        scorecards_kit.router(),
    )
}
