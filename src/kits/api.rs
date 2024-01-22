use axum::Router;

use crate::io::routable::Routable;

pub async fn router(kits: Vec<impl Routable>) -> Router {
    let mut merged_router = Router::new();

    for kit in kits.iter() {
        merged_router = merged_router.merge(kit.router());
    }

    Router::new().nest(
        &format!("/{}", module_path!().split("::").last().unwrap()),
        merged_router,
    )
}
