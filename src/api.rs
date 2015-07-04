use ::rustc_serialize::json::Json;

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::collections::HashMap;

/// Execution error struct
#[derive(Debug)]
pub struct CallError {
    pub response: String,
    pub because: Option<Box<Error>>
}

impl CallError {
    pub fn new(response: String, because: Option<Box<Error>>) -> CallError {
        CallError { response: response, because: because}
    }
}

impl Display for CallError {

    fn fmt(&self,f: &mut Formatter) -> Result<(), ::std::fmt::Error> {
        format!("Invalid response from VK API: {}", self.response).fmt(f)
    }
}

impl Error for CallError {

    fn description(&self) -> &str {
        "Invalid response from VK API."
    }

    fn cause(&self) -> Option<&Error> {
        match self.because {
            Some(ref e) => Some(&**e),
            None => None
        }
    }

}

pub trait VkApi {
    fn call(&self, method: String, params: HashMap<String, String>) -> Result<Json, CallError>;
}
