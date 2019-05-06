pub mod prelude {
    pub use super::HttpStatus;
}

pub enum HttpStatus {
    Ok,
    NotFound,
}

impl HttpStatus {
    pub fn code(&self) -> u32 {
        match self {
            HttpStatus::Ok => 200,
            HttpStatus::NotFound => 404,
        }
    }

    pub fn msg(&self) -> &str {
        match self {
            HttpStatus::Ok => "OK",
            HttpStatus::NotFound => "Not Found",
        }
    }

    pub fn status_line(&self) -> String {
        const STATUS_LINE_PREFIX: &str = "HTTP/1.1";
        format!("{} {} {}", STATUS_LINE_PREFIX, self.code(), self.msg())
    }
}
