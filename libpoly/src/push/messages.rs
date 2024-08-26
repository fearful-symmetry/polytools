use digest_auth::AuthContext;
use reqwest::{blocking, header::{AUTHORIZATION, CONTENT_TYPE}, Method};

use crate::{errors::PolyRestError, push::{MessageData, PolycomIPPhone}};

use super::{MessageLevel, PushMessenger, PushType};


impl PushMessenger {
    /// Create a new push messenger
    /// Note that the push API credentials are often different from the REST API credentials.
    pub fn new<S: Into<String>>(username: S, password: S, url: S, insecure: bool) -> Result<Self, PolyRestError> {
        let client = blocking::Client::builder().danger_accept_invalid_certs(insecure).build()?;
        Ok(Self { client, username: username.into(), password: password.into(), url: url.into()})
    }

    /// Send a one-time message
    /// The message_body will be inserted into the Data object of the XML push body.
    pub fn send<S: Into<String>>(&mut self, level: MessageLevel, message_body: S, cmd_type: PushType) -> Result<String, PolyRestError> {
        let payload = PolycomIPPhone {data: MessageData{priority: level, body: message_body.into()}};
        self.send_message_payload(payload, cmd_type)
    } 

    // here be dragons
    fn send_message_payload(&mut self, payload: PolycomIPPhone, command_type: PushType) -> Result<String, PolyRestError> {
        let str_payload = quick_xml::se::to_string(&payload)?;
        //unfuck XML
        let unescaped = quick_xml::escape::unescape(&str_payload)?.to_string();
        let path = format!("{}/push", self.url);
        // for whatever reason, the push endpoint requires digest auth, while  the regular rest API doesn't, so go through the digest auth steps.
        //  the Polycom API server is....particular. Do this in some way it doesn't like and it'll just return a 200 and silently fail.
        let test_req = self.client.request(Method::POST, path.clone()).header(CONTENT_TYPE, "application/x-www-form-urlencoded").build()?;
        let test_resp = self.client.execute(test_req)?;

        // fetch the auth headers from the first response
        let www_auth = test_resp.headers().get("www-authenticate").unwrap();
        let context = AuthContext::new_with_method(&self.username, &self.password, path.clone(), Some(unescaped.as_bytes()), digest_auth::HttpMethod::from("POST"));
        let mut prompt = digest_auth::parse(www_auth.to_str().unwrap()).unwrap();
        let new_headers = prompt.respond(&context).unwrap();

        // send the second request
        let second_req = self.client.request(Method::POST, path.clone())
        .body(unescaped).header(CONTENT_TYPE, command_type).header(AUTHORIZATION, new_headers.to_header_string()).build()?;
        let second_resp = self.client.execute(second_req)?.error_for_status()?; 

        let raw_resp = second_resp.bytes()?;   
        let resp_str = String::from_utf8_lossy(&raw_resp).to_string();

        Ok(resp_str)
    }
}

