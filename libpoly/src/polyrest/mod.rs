//! Handlers for the Polycom REST API.

use reqwest::{blocking, Method};
use crate::errors::PolyRestError;

pub mod mgmt;
/// An API handler for dealing with Polycom's REST API.
/// 
/// This API serves a number of control and management functions, allowing the user to place calls, set and get config, and state, etc.
/// As of this writing, the API is incomplete.
/// 
/// ```
/// use libpoly::polyrest::PolyRest;
/// 
/// let mut handler = PolyRest::new("Polycom", "789", "https://192.168.1.9", true).unwrap();
/// let info = handler.device_info().unwrap();
/// println!("device info: {:?}", info);
/// ```
pub struct PolyRest {
    username: String,
    password: String,
    url: String,
    client: reqwest::blocking::Client
}

impl PolyRest {
    /// Create a new PolyRest handler 
    pub fn new<S: Into<String>>(username: S, password: S, url: S, insecure: bool) -> Result<Self, PolyRestError> {
        let client = blocking::Client::builder().danger_accept_invalid_certs(insecure).build()?;
        Ok(Self { client, username: username.into(), password: password.into(), url: url.into()})
    }

    fn raw_get(&mut self, path: String) -> Result<String, PolyRestError> {
        let req = self.client.request(Method::GET, path).basic_auth(self.username.clone(), Some(self.password.clone())).build()?;
        let resp = self.client.execute(req)?.error_for_status()?;
        let raw_resp = resp.bytes()?;   
        
        let resp_str = String::from_utf8_lossy(&raw_resp);

        Ok(resp_str.to_string())   
    }
}