use axum::Router;

pub fn router() -> Router {
    Router::new().nest("/api", Router::new())
}
