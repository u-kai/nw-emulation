use std::collections::HashMap;

use super::macaddr::MacAddr;

pub struct MacAddressTable {
    table: HashMap<Port, MacAddr>,
}
struct Port(String);

impl MacAddressTable {
    pub fn new() -> Self {
        MacAddressTable {
            table: HashMap::new(),
        }
    }
    //pub fn connect(&mut self,port:)
}
