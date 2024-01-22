use axum::Router;

pub trait Routable {
    fn router(&self) -> Router;
}
