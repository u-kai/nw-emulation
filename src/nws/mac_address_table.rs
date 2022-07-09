use std::collections::HashMap;

use super::macaddr::MacAddr;

pub struct MacAddressTable {
    table: HashMap<L2Port, MacAddr>,
}
impl MacAddressTable {
    pub fn get_port(&self, mac_addr: MacAddr) -> Option<&L2Port> {
        self.table
            .iter()
            .find(|(port, addr)| addr == &&mac_addr)
            .map(|(port, _addr)| port)
    }
    pub fn register_record(&mut self, source_addr: MacAddr, source_port: L2Port) {
        self.table.insert(source_port, source_addr);
    }
    pub fn reset(&mut self) {
        self.table.clear();
    }
}
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct L2Port(String);
impl L2Port {
    pub fn new() -> Self {
        L2Port(String::new())
    }
}

impl MacAddressTable {
    pub fn new() -> Self {
        MacAddressTable {
            table: HashMap::new(),
        }
    }
    //pub fn connect(&mut self,port:)
}
