use std::fmt::Display;

impl Display for FileDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{file_path: {}, title: {}}}",
            self.file_path, self.title
        )
    }
}

pub struct FileDisplay {
    pub file_path: String,
    pub title: String,
}
