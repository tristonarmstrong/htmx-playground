use askama::Template;

use crate::structs::FileDisplay;

#[derive(Template)]
#[template(path = "portfolio.html")]
pub struct Portfolio;

#[derive(Template)]
#[template(path = "notes.html")]
pub struct Notes;

#[derive(Template)]
#[template(path = "tuts.html")]
pub struct Tuts {
    pub tuts_list: Vec<FileDisplay>,
}

#[derive(Template)]
#[template(path = "tut.html", ext = "html", escape = "none")]
pub struct Tut<'a> {
    pub title: &'a str,
    pub content: &'a str,
}
