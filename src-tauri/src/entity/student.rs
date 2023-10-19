use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Student {
    pub name: String,
    pub age: i32,
}
