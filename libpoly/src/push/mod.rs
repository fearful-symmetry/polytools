//! Handlers for the /push API for sending HTML to the Polycom's built-in web browser.

use std::fmt::Display;
use clap::builder::PossibleValue;
use reqwest::header::{HeaderValue, InvalidHeaderValue};
use serde::{Deserialize, Serialize};

mod messages;

/// The type of push command to use.
/// 
/// This determines the Content-Type header sent to the /push API,
/// and determines how the phone will parse the API event. For example,
/// if you send `Key:Home` with the `PushType::Command` enum, it'll merely 
/// print the string `Key:Home` to the phone's web browser.
#[derive(Clone, Debug)]
pub enum PushType {
    /// Send as an HTML event, the resulting event body will pop up on the phone's browser
    HTML,
    /// Parse the body as a command
    Command
}

impl Display for PushType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PushType::Command => write!(f, "cmd"),
            PushType::HTML => write!(f,"html")
        }
    }
}

impl TryFrom<PushType> for HeaderValue {
    type Error = InvalidHeaderValue;
    fn try_from(value: PushType) -> Result<Self, Self::Error> {
        let str_header = match value {
            PushType::HTML => String::from("application/x-www-form-urlencoded"),
            PushType::Command => String::from("application/x-com-polycom-spipx")
        };
        str_header.try_into()
    }
}

impl clap::ValueEnum for PushType {
    fn value_variants<'a>() -> &'a [Self] {         
        &[Self::HTML, Self::Command]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            PushType::HTML => Some(PossibleValue::new("html")),
            PushType::Command => Some(PossibleValue::new("cmd")),
        }
    }
}


/// The message level to send the notification at
#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum MessageLevel {
    Critical,
    High,
    Important,
    Normal
}

impl clap::ValueEnum for MessageLevel {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Critical, Self::High, Self::Important, Self::Normal]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            MessageLevel::Critical => Some(PossibleValue::new("critical")),
            MessageLevel::High => Some(PossibleValue::new("high")),
            MessageLevel::Important => Some(PossibleValue::new("important")),
            MessageLevel::Normal => Some(PossibleValue::new("normal"))
        }
    }
}

impl Display for MessageLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageLevel::Critical => write!(f, "critical"),
            MessageLevel::High => write!(f, "high"),
            MessageLevel::Important => write!(f, "important"),
            MessageLevel::Normal => write!(f, "normal")
        }
    }
}


#[derive(Deserialize, Serialize, Debug)]
struct PolycomIPPhone {
    #[serde(rename="Data")]
    pub data: MessageData
}

#[derive(Deserialize, Serialize, Debug)]
struct MessageData {
    #[serde(rename="@priority")]
    pub priority: MessageLevel,
    #[serde(rename="$text")]
    pub body: String
}



#[derive(Clone)]
/// A PushMessenger creates and sends events to the /push API on the Polycom device.
/// 
/// The push API is rather idiosyncratic; it uses Digest Auth and expects its own formatted XHTML body.
/// This library wraps most of that behavior, allowing a user to just send HTML:
/// 
/// ```
/// use libpoly::push::{PushMessenger, PushType, MessageLevel};
/// 
/// let mut handle = PushMessenger::init("Push", "Push", "https://192.168.1.9", true).unwrap();
///handle.send(MessageLevel::Critical, "<h1>Silence Mortal, the VOIP phone is speaking </h1>", PushType::HTML).unwrap();
/// ```
pub struct PushMessenger {
    username: String,
    password: String,
    url: String,
    client: reqwest::blocking::Client
}
