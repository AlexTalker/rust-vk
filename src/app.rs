//! # VkApp
//! Presentation of application to easily realizing authorization process.
//!
//! # Examples
//! ```ignore
//! let app = VkApp::new(3252132, "v5.34".into(), Some("http://example.com/vk-redirect.html".into()));
//! let url = app.auth_client_uri("audio, friends".into());
//! //... And then redirect user on the url and so...
//! ```

extern crate hyper;

use std::io::prelude::*;
use ::rustc_serialize::json::Json;

use api::CallError;
use user::VkUser;
use server::VkServer;
use fake_browser::{authorization_client_uri, fake_browser};

/// Reflect VK app needs
#[derive(Debug)]
pub struct VkApp {
    app_id: u64,
    version: String,
    redirect: String
}

impl VkApp {

    /// Constructor of new app.
    pub fn new(app_id: u64, version: String, mut redirect: Option<String>) -> VkApp {
        if redirect.is_none() {
            redirect = Some("https://oauth.vk.com/blank.html".into());
        }
        VkApp { app_id: app_id, version: version, redirect: redirect.unwrap() }
    }

    /// Return URL for user(client, Standalone) authorization. [Documentation
    /// overview](https://vk.com/dev/auth_mobile)
    pub fn auth_client_uri(&self, scope: String) -> String {
        authorization_client_uri(self.app_id, scope, self.version.clone(), self.redirect.clone())
    }

    /// Build URL to redirect a user in Authorization Code Flow OAuth
    /// authrization case.
    pub fn auth_site_uri(&self, scope: String, state: String) -> String {
        format!("https://oauth.vk.com/authorize?client_id={id}&scope={scope}&redirect_uri={redirect}&response_type=code&v={v}&state={state}", id=self.app_id, scope=scope, redirect=self.redirect, v=self.version,state=state)
    }

    /// Implement client authorization without using real user browser.
    /// __Warning:__ Use the method only for test or with care about user privacy
    /// and vk.com privacy policy. Use it on your own risk 'cause there's no guarantee
    /// that the way will work always success.
    pub fn client(&self, login: String, pass: String, scope: String ) -> Result<VkUser, CallError> {
        match fake_browser(login, pass, self.auth_client_uri(scope)) {
            Ok((token, expires, id)) => Ok(VkUser::new(id, token, expires)),
            Err(e) => Err(e)
        }
    }
    /// Implement Server authorization for 'secure.*' methods of VK.com API.
    pub fn server(&self, secret: String) -> Result<VkServer, CallError> {
        let url = format!("https://oauth.vk.com/access_token?client_id={}&client_secret={}&v={}&grant_type=client_credentials", self.app_id, secret, self.version);
        match VkApp::fetch_json(url) {
            Ok(object) => {
                match object.find("access_token") {
                    Some(json) => {
                        if json.is_string() {
                            Ok(VkServer::new(
                                    self.app_id,
                                    json.as_string().unwrap().to_string(),
                                    secret))
                        }
                        else {
                            Err(CallError::new("Error parse json.".into(), None))
                        }
                    }
                    None => {
                        Err(CallError::new("Error access_token field".into(), None))
                    }
                }
            }
            Err(e) => {
                Err(CallError::new("Cannot fetch JSON.".into(), Some(Box::new(e))))
            }
        }
    }
    /// Function that build new VkUser instance using use code and secret of the app.
    /// (Authorization Code Flow)
    pub fn site(&self, secret: String, code: String) -> Result<VkUser,CallError> {

        let url = format!("https://oauth.vk.com/access_token?client_id={id}&client_secret={secret}&code={code}&redirect_uri={redirect}", id=self.app_id, secret=secret,code=code, redirect=self.redirect);

        match VkApp::fetch_json(url) {
            Ok(object) => {
                if object["access_token"].is_string() && object["expires_in"].is_u64() && object["user_id"].is_u64() {
                   Ok(VkUser::new(
                       object["user_id"].as_u64().unwrap(),
                       object["access_token"].as_string().unwrap().into(),
                       object["expires_in"].as_u64().unwrap()))
                }
                else {
                    Err(CallError::new("access_token, expires_in or user_id field is missing in answered object.".into(), None))
                }
            }
            Err(e) => {
                Err(CallError::new("Cannot fetch JSON.".into(), Some(Box::new(e))))
            }
        }


    }

    fn fetch_json(url: String) -> Result<Json, CallError> {
        use self::hyper::client::Client;
        use self::hyper::client::response::Response;

        let client = Client::new();

        let mut res: Response;

        match client.get(&url).send() {
            Ok(r) => res = r,
            Err(e) => return Err(CallError::new("Can't make a request to get JSON.".into(), Some(Box::new(e))))
        };

        let mut answer = String::new();

        match res.read_to_string(&mut answer) {
            Ok(_) => {
                match answer.parse::<Json>() {
                    Ok(object) => {
                        Ok(object)
                    }
                    Err(e) => {
                        Err(CallError::new(
                                "Cannot parse answer to JSON object.".into(),
                                Some(Box::new(e))))
                    }
                }
            }
            Err(e) => {
                Err(CallError::new(
                        "Cannot read server answer.".into(),
                        Some(Box::new(e))))
            }
        }

    }

}
