pub mod prelude {
    pub use super::HtmlFile;
    pub use super::Response;
}

use std::fs;
use std::path::Path;

use super::status::HttpStatus;
use crate::HTML_DIR;

pub enum Response {
    File(HttpStatus, HtmlFile),
    Text(HttpStatus, String),
}

impl Response {
    pub fn raw(&self) -> String {
        match self {
            Response::File(status, file) => {
                format!("{}\r\n\r\n{}", status.status_line(), file.contents())
            }
            Response::Text(status, text) => {
                format!("{}\r\n\r\n{}", status.status_line(), text)
            }
        }
    }
}

pub struct HtmlFile(String);

impl HtmlFile {
    pub fn new<T>(filename: T) -> Self
    where
        T: ToString,
    {
        Self(path!(HTML_DIR, filename).to_str().unwrap().to_string())
    }

    pub fn contents(&self) -> String {
        fs::read_to_string(&self.0).unwrap()
    }
}
