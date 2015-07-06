//! # VkServer
//! The module implement simple interface to
//! server-only methods using secret token of your app(Client Credentials Flow).
use ::rustc_serialize::json::Json;

use std::collections::HashMap;

use api::{VkApi,CallError};
use execute::execute;

/// Structure to handle calls to 'secure.*' API methods(Client Credentials Flow).
#[derive(Debug,Clone)]
pub struct VkServer{
    id: u64,
    access_token: String,
    secret: String
}

impl VkServer {
    /// Secret is your secret token of application
    pub fn new(id: u64, access_token: String, secret: String) -> VkServer {
        VkServer { id: id, access_token: access_token, secret: secret }
    }

}

impl VkApi for VkServer {

    fn call(&self, method: String, mut params: HashMap<String, String>) -> Result<Json, CallError>{
        if !params.contains_key("client_secret") {
            params.insert("client_secret".into(), self.secret.clone());
        }

        execute(method, params, self.access_token.clone())
    }
}
