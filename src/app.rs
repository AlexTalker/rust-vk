//! # VkApp
//! Presentation of application to easily realizing authorization process.
//!
//! # Examples
//! ```rustc
//! let app = VkApp::new(3252132, "v5.34".into(), Some("http://example.com/vk-redirect.html".into()));
//! let url = app.auth_client_uri("audio, friends".into());
//! //... And then redirect user on the url and so...
//! ```

use api::CallError;
use user::VkUser;
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

}
