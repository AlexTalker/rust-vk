#[derive(Debug, Clone)]
pub struct VkUser {
    pub id: u64,
    access_token: String,
    pub expires_in: u64
}

impl VkUser {
    /// Construct new user by ID, access token and time that the token will be expired.
    pub fn new(id: u64, token: String, expires: u64) -> VkUser {
        VkUser { id: id, access_token: token, expires_in: expires }
    }

}

use api::{VkApi, CallError};
use execute::execute;
use ::rustc_serialize::json::Json;
use std::collections::HashMap;

impl VkApi for VkUser {
    fn call(&self, method: String, params: HashMap<String, String>) -> Result<Json, CallError> {
        execute(method, params, self.access_token.clone())
    }
}
