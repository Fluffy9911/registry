use serde::{Deserialize, Serialize};
use crate::key;
use crate::key::{Key, KeyStr};


#[derive(Serialize, Deserialize)]
pub struct Event {
key:KeyStr,
    event_type:KeyStr,
max_listeners:usize
}

pub fn new_event_type(id:&str,type_key:&str) -> KeyStr{

     let k = key::Key::new(id.parse().unwrap(), type_key.parse().unwrap());
  KeyStr(k.to_key_string())

}






