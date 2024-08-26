//! Libpoly is a set of APIs for sending events and messages to Polycom phones. It currently supports the Push and REST APIs found on VVX-line Polycom VoIP phones.
//! ```
//! use libpoly::polyrest::PolyRest;
//! 
//! let mut handler = PolyRest::new("Polycom", "789", "https://192.168.1.9", true).unwrap();
//! let info = handler.device_info().unwrap();
//! println!("device info: {:?}", info);
//! ```

use polyrest::PolyRest;

pub mod polyrest;
pub mod push;
pub mod errors;


