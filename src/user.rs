//! # VkUser
//! Represent user abstraction to easily handling calls to VK.com API.
//!
//! # Examples
//!
//! ```ignore
//! use std::collections::HashMap;
//!
//! let user = VkUser::new(3424133213, "dshwi3092ew98uihedsyu348eihw9832buds8hnekjnsdbfeyusdj",
//! 84000);
//! match user.call("friends.getOnline", HashMap::<String,String>::new()) {
//!     Ok(json) => ..., // Handling success result(with included json["response"] field of the JSON object)
//!     Err(e) => ... // Handling API and request errors.
//! }
//! ```
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
    /// Make call to VK.com API by the method with the params and user access token.
    fn call(&self, method: String, params: HashMap<String, String>) -> Result<Json, CallError> {
        execute(method, params, self.access_token.clone())
    }
}
