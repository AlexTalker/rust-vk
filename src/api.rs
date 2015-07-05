//! # Vk API
//! The module contain implementation of trait to standardize
//! calls to VK.com API and abstract at access details.
//! Also contain CallError that's wrapper around errors of call execution process.
use ::rustc_serialize::json::Json;

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::collections::HashMap;

/// Execution error struct
#[derive(Debug)]
pub struct CallError {
    /// Data of response that throw the error
    pub response: String,
    /// Error that call the error(like JSON parsing error or Read trait error).
    pub because: Option<Box<Error>>
}

impl CallError {
    /// Constructor new copy of the error structure.
    /// __response:__ Data that are reason to throw the error.
    /// __because:__ Other error that was reason to throw this one.
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

/// Abstraction to make calls to VK.com API independently at implementor structure.
pub trait VkApi {
    /// Must be implemented by all structures that realize interface to VK.com API calls.
    fn call(&self, method: String, params: HashMap<String, String>) -> Result<Json, CallError>;
}
