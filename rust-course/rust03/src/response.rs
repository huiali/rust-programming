use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBody<T> {
    pub message: String,
    pub data: T,
    pub status: u32,
}

impl<T> ResponseBody<T> {
    pub fn new(message: &str, data: T, status: u32) -> ResponseBody<T> {
        ResponseBody {
            message: message.to_string(),
            data,
            status,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PageResponse<T> {
    pub items: Vec<T>,
    pub count: u64,
}
