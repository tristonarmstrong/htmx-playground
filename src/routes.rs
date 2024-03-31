use askama::Template;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use comrak::markdown_to_html;

use crate::{
    errors::ApiError,
    structs::FileDisplay,
    templates::{self, Tut},
    utils,
};

pub async fn portfolio() -> impl IntoResponse {
    templates::Portfolio
}

pub async fn notes() -> impl IntoResponse {
    templates::Notes
}

pub async fn tuts(paths: Vec<String>) -> impl IntoResponse {
    let mut new_paths: Vec<FileDisplay> = vec![];
    for path in paths {
        let new_struct = FileDisplay {
            file_path: path.clone().to_string(),
            title: utils::upper_all_words_first_letters(utils::rm_delimeter(
                utils::rm_path_ext(path),
                "_".to_string(),
                " ".to_string(),
            ))
            .unwrap(),
        };

        new_paths.push(new_struct);
    }

    templates::Tuts {
        tuts_list: new_paths,
    }
}

pub async fn tuts_builder(path: String, tut_title: String) -> Result<impl IntoResponse, ApiError> {
    let file_contents = std::fs::read_to_string(path).unwrap();
    let file_contents_to_html =
        markdown_to_html(file_contents.as_str(), &comrak::Options::default());
    let file_template = Tut {
        title: &tut_title,
        content: file_contents_to_html.as_str(),
    };
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(file_template.render().unwrap())
        .unwrap();

    Ok(response)
}

pub async fn styles() -> Result<impl IntoResponse, ApiError> {
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/css")
        .body(include_str!("../templates/styles.css").to_owned())?;

    Ok(response)
}
