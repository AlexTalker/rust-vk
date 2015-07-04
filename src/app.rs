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

    /// Constructor new app.
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

    pub fn client(&self, login: String, pass: String, scope: String ) -> VkUser {
        let (token, expires, id) = fake_browser(login, pass, self.auth_client_uri(scope));
        VkUser::new(id, token, expires)
    }

}
