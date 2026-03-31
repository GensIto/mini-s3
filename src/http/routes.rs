use axum::Router;
use axum::routing::{get, head};

use crate::http::handlers;
use crate::http::state::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(handlers::buckets::list_buckets))
        .route(
            "/{bucket}",
            get(handlers::objects::list_or_head_bucket)
                .put(handlers::buckets::create_bucket)
                .delete(handlers::buckets::delete_bucket),
        )
        .route(
            "/{bucket}/{*key}",
            head(handlers::objects::head_object_keyed)
                .get(handlers::objects::get_object_keyed)
                .put(handlers::objects::put_object_keyed)
                .delete(handlers::objects::delete_object_keyed),
        )
        .with_state(state)
}
