use axum::{routing::get, Router};

use crate::routes;

#[derive(Clone)]
pub struct AppState {}

// put all notes routes here
pub fn init_notes_routes() -> Router {
    Router::new().route("/", get(routes::notes))
}

// put all tutorial routes here
pub fn init_tuts_routes() -> Router {
    Router::new().route("/", get(routes::tuts))
}

// put any non nested routes here
pub fn init_utily_routes() -> Router {
    Router::new()
        .route("/", get(routes::portfolio))
        .route("/styles.css", get(routes::styles))
}

pub fn init_router() -> Router {
    let routes: Router<()> = Router::<()>::new()
        .nest("/tuts", init_tuts_routes())
        .nest("/notes", init_notes_routes())
        .nest("/", init_utily_routes());

    Router::new().nest("/", routes)
}
