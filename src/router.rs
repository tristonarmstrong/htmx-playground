use axum::{routing::get, Router};
use std::{env, fs};

use crate::routes;

#[derive(Clone)]
pub struct AppState {}

// put all notes routes here
pub fn init_notes_routes() -> Router {
    Router::new().route("/", get(routes::notes))
}

pub fn update_route(router: Router, file_name_str: String, file_path_str: String) -> Router {
    let route = format!("/{}", file_name_str);
    router.route(
        route.clone().as_str(),
        get(move || routes::tuts_builder(file_path_str)),
    )
}
// put all tutorial routes here
pub fn init_tuts_routes() -> Router {
    let mut router = Router::new().route("/", get(routes::tuts));
    let path = env::current_dir().unwrap();
    let mut path_string = path.clone().into_os_string();
    path_string.push("/tuts");
    for entry in fs::read_dir(path_string).unwrap() {
        let entry = entry.unwrap();
        let file_path = entry.path().to_str().unwrap().to_string();
        let file_name = entry.file_name();
        let file_name_str = file_name.to_str().unwrap().to_string();
        router = update_route(router, file_name_str, file_path);
    }

    router
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
