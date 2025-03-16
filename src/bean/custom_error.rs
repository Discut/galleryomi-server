use std::io::Cursor;
use rocket::http::Status;
use rocket::response::Responder;
use rocket::Request;
use rocket::Response;

// 自定义错误处理
#[derive(Debug)]
pub struct CustomError {
    pub status: Status,
    pub message: String,
}

impl CustomError {
    pub fn new(status: Status, message: impl Into<String>) -> Self {
        Self {
            status,
            message: message.into(),
        }
    }
}

impl From<diesel::result::Error> for CustomError {
    fn from(e: diesel::result::Error) -> Self {
        match e {
            diesel::result::Error::NotFound => {
                Self::new(Status::NotFound, "资源未找到")
            }
            _ => Self::new(Status::InternalServerError, "数据库错误"),
        }
    }
}

impl<'r> Responder<'r, 'static> for CustomError {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        Response::build()
            .status(self.status)
            .sized_body(None, Cursor::new(self.message))
            .ok()
    }
}