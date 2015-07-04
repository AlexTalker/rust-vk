extern crate hyper;
extern crate rustc_serialize;

use self::hyper::client::Client;

use self::rustc_serialize::json::Json;

use api::CallError;

use std::io::prelude::*;
use std::collections::HashMap;

/// Method that execute VK API method call with access token
pub fn execute(method: String, params: HashMap<String, String>, token: String) -> Result<Json, CallError> {
    let params_get = params.iter().fold(String::new(), |s, (key, value)| { s + &format!("{}={}&",key,value) });
    let url = format!("https://api.vk.com/method/{}?{}access_token={}", method, params_get, token);
    let client = Client::new();
    let mut res = client.get(&url).send().unwrap();
    let mut answer = String::new();
    match res.read_to_string(&mut answer) {
        Ok(_) => {
            match answer.parse::<Json>() {
                Ok(json) => { 
                    if json.find("error").is_some() {
                        Err(CallError::new(json["error"].to_string(), None))
                    }
                    else {
                        Ok(json)
                    }
                },
                Err(e) => Err(CallError::new(format!("{}", answer), Some(Box::new(e))))
            }
        },
        Err(e) => {
            Err(CallError::new(format!("{:?}", res), Some(Box::new(e))))
        }
    }
}
