use askama::Template;

#[derive(Template)]
#[template(path = "portfolio.html")]
pub struct Portfolio;

#[derive(Template)]
#[template(path = "notes.html")]
pub struct Notes;

#[derive(Template)]
#[template(path = "tuts.html")]
pub struct Tuts;
