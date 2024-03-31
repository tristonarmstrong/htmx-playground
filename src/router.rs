use axum::{routing::get, Router};
use std::{env, fs};

use crate::{routes, utils};

const FILE_NAME_DELIMETER: &str = "_";
const FILE_NAME_DELIMETER_REPLACEMENT: &str = " ";

#[derive(Clone)]
pub struct AppState {}

// put all notes routes here
pub fn init_notes_routes() -> Router {
    Router::new().route("/", get(routes::notes))
}

pub fn update_tuts_route(router: Router, file_name_str: String, file_path_str: String) -> Router {
    let route = format!("/{}", file_name_str);
    let tut_title = utils::rm_path_ext(utils::rm_delimeter(
        file_name_str,
        FILE_NAME_DELIMETER.to_string(),
        FILE_NAME_DELIMETER_REPLACEMENT.to_string(),
    ));

    router.route(
        route.clone().as_str(),
        get(move || {
            routes::tuts_builder(
                file_path_str,
                utils::upper_all_words_first_letters(tut_title).unwrap(),
            )
        }),
    )
}

// put all tutorial routes here
pub fn init_tuts_routes() -> Router {
    let mut current_dir = env::current_dir().unwrap().into_os_string();
    current_dir.push("/tuts");
    let directory = fs::read_dir(current_dir).unwrap();
    let mut file_dir_vec = Vec::<String>::new();

    let mut router = Router::new();
    for file in directory {
        let entry = file.unwrap();
        let file_path = entry.path().to_str().unwrap().to_string();
        let file_name = entry.file_name();
        let file_name_str = file_name.to_str().unwrap().to_string();
        file_dir_vec.push(file_name_str.clone());
        router = update_tuts_route(router, file_name_str, file_path);
    }

    router = router.route("/", get(move || routes::tuts(file_dir_vec)));

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

    Router::new()
        .nest("/", routes)
        .layer(tower_livereload::LiveReloadLayer::new())
}
