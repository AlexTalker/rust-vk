extern crate cookie;
use self::cookie::CookieJar;

extern crate regex;
use self::regex::Regex;

extern crate hyper;
use self::hyper::client::{Client, RedirectPolicy};
use self::hyper::Url;
use std::io::prelude::*;

use std::error::Error;
use std::fmt::{Display, Formatter};

use api::CallError;

/// Function that return authorization uri for Standalone client
pub fn authorization_client_uri(client_id: u64, scope: String, version: String, redirect: String) -> String {
    format!("https://oauth.vk.com/authorize?client_id={}&scope={}&redirect_uri={}&display=mobile&v={}&response_type=token", client_id, scope, redirect, version)
}

use std::collections::HashMap;
// Get params send by hidden fields on auth page form
fn hidden_params(s: String) -> HashMap<String,String> {
    let mut map = HashMap::new();
    let reg = Regex::new("name=\"([a-z_]*)\".*value=\"([:A-Za-z-/0-9.]+)\"").unwrap();
    for cap in reg.captures_iter(&s) {
        map.insert(cap.at(1).unwrap_or("").into(), cap.at(2).unwrap_or("").into());
    }
    map
}
// Build POST request body for <form>
fn build_post_for_hidden_form(mut hidden_fields: HashMap<String,String>, login: String, password: String) -> String {
    let mut result = String::new();
    hidden_fields.insert("email".into(), login);
    hidden_fields.insert("pass".into(), password);
    for (key, value) in hidden_fields.iter() {
        result.extend( format!("{}={}&", key,value).chars() );
    }
    result
}
// Find URL to send auth form
fn get_post_uri(s: String) -> String {
    let reg = Regex::new("action=\"([a-z:/?=&.0-9]*)\"").unwrap();
    match reg.captures_iter(&s).next() {
        Some(x) => x.at(1).unwrap_or(""),
        None => ""
    }.into()
}
// Get access token and other data from response URL
fn get_token(u: Url) -> (String, u64, u64) {
    let reg = Regex::new("access_token=([a-f0-9]+)&expires_in=([0-9]+)&user_id=([0-9]+)").unwrap();
    let mut token: String = String::new();
    let mut expires: u64 = 0u64;
    let mut user_id: u64 = 0u64;
    for cap in reg.captures_iter(&u.to_string()) {
        token = cap.at(1).unwrap_or("").into();
        expires = cap.at(2).unwrap_or("0").parse::<u64>().unwrap();
        user_id = cap.at(3).unwrap_or("0").parse::<u64>().unwrap();
    }
    (token, expires, user_id)
}
// Find url to confirm rights after authorization process(not always showed form)
fn find_confirmation_form(s: &String) -> String {
    let mut result = String::new();
    let reg = Regex::new("action=\"([A-Za-z0-9:/.?=&_%]+)\"").unwrap();
    for cap in reg.captures_iter(&*s) {
        result = cap.at(1).unwrap_or("").into();
    }
    result
}
// Stub
fn detect_captcha(s: &String) -> bool {
    let reg = Regex::new("id=\"captcha\"").unwrap();
    if reg.is_match(&*s) {
        true
    }
    else{
        false
    }
}

/// Error returned if captcha was detected on login process
/// _Warning:_ the error isn't about 'Captcha needed' VK.com API real error.
#[derive(Debug)]
pub struct CapthaError;

impl Display for CapthaError {

    fn fmt(&self,f: &mut Formatter) -> Result<(), ::std::fmt::Error> {
        "Captcha was found on authorization process.".fmt(f)
    }

}

impl Error for CapthaError {

    fn description(&self) -> &str {
        "Captha was found on authorization process."
    }

}
/// The function implement login process for user without browser
/// _Warning: use the thing careful to privacy and privacy policy of vk.com_
pub fn fake_browser(login: String, password: String, url: String) -> Result<(String, u64, u64),CallError> {
    use std::thread::sleep_ms;
    use self::hyper::header::{Cookie,Location,SetCookie, ContentLength};
    let mut client = Client::new();
    client.set_redirect_policy(RedirectPolicy::FollowNone);
    let mut res = client.get(&url).send().unwrap();
    let mut jar = CookieJar::new(b"");
    res.headers.get::<SetCookie>().unwrap().apply_to_cookie_jar(&mut jar);
    let mut result = String::new();
    res.read_to_string(&mut result).unwrap();
    let params = hidden_params(result.clone());
    sleep_ms(1000);
    let post_req = build_post_for_hidden_form(params, login, password);
    let post_uri = get_post_uri(result.clone());
    res = client.post(&post_uri).header::<Cookie>(Cookie::from_cookie_jar(&jar)).body(&post_req).send().unwrap();
    while res.headers.has::<Location>() {
        if res.headers.has::<SetCookie>() {
            res.headers.get::<SetCookie>().unwrap().apply_to_cookie_jar(&mut jar);
        }
        let redirect = res.headers.get::<Location>().unwrap().clone();
        res = client.get(&*redirect).header::<Cookie>(Cookie::from_cookie_jar(&jar)).send().unwrap();
        let length = res.headers.get::<ContentLength>().unwrap().clone();
        // Check that we've got yet one confirmation form
        if length != ContentLength(0u64) {
            let mut answer = String::new();
            if let Ok(_) = res.read_to_string(&mut answer) {
                if detect_captcha(&answer) {
                    return Err(CallError::new(answer, Some(Box::new(CapthaError))));
                }
                let url = find_confirmation_form(&answer);
                if !url.is_empty() {
                    res = client.post(&url).header::<Cookie>(Cookie::from_cookie_jar(&jar)).send().unwrap();
                }
            }

        }
    }
    Ok(get_token(res.url.clone()))
}
