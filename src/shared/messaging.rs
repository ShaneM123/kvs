use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
pub struct GetStream <'a,'b> {
    pub cmd: &'a str,
    pub key: &'b str,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetStream <'a,'b, 'c> {
    pub cmd: &'a str,
    pub key: &'c str,
    pub value: &'b str,
}
