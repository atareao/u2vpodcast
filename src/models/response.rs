use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    id: i32,
    message: String,
    content: Value,
}

impl Response {
    pub fn new(id: i32, message: &str, content: Value) -> Self{
        Self{
            id,
            message: message.to_string(),
            content,
        }
    }
    pub fn get_json_as_string(&self) -> String{
        serde_json::to_string(self).unwrap()
    }
    
}
