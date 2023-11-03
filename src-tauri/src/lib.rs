use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DataResponse<T> {
    code: u32,
    data: Option<T>,
    message: String,
}

impl<T> DataResponse<T> {
    pub fn success(data: T) -> Self {
        DataResponse {
            code: 200,
            data: Some(data),
            message: String::from("请求成功"),
        }
    }
    pub fn failure(message: String) -> Self {
        DataResponse {
            code: 404,
            data: None,
            message,
        }
    }

    pub fn fast_failure() -> Self {
        DataResponse {
            code: 404,
            data: None,
            message: String::from("请求失败"),
        }
    }

    pub fn fast_success() -> Self {
        DataResponse {
            code: 200,
            data: None,
            message: String::from("请求成功"),
        }
    }
}