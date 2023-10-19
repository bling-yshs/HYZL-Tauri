use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
pub struct ReturnData<'a, T> {
    pub code: i32,
    pub data: T,
    pub message: &'a str,
}

impl<'a> ReturnData<'a, ()> {
    pub fn fast_failure(message: &str) -> String {
        json!(
            ReturnData {
            code: 500,
            data: (),
            message,
        }).to_string()
    }
    pub fn fast_success(message: &str) -> String {
        json!(
            ReturnData {
            code: 200,
            data: (),
            message,
        }).to_string()
    }
    // pub fn failure(code: i32, message: &str) -> ReturnData<()> {
    //     ReturnData {
    //         code,
    //         data: (),
    //         message,
    //     }
    // }
}
