use std::{collections::HashMap, ops::Deref};

use super::{
    ipv4::IpAddrv4,
    mac_address_table::{L2Port, MacAddressTable},
    macaddr::MacAddr,
};

pub struct L2Switch<T> {
    ip: IpAddrv4,
    mac: MacAddr,
    connected_nodes: Vec<ConnectedNode<T>>,
    mac_addr_table: MacAddressTable,
}

pub struct ConnectedNode<T>(HashMap<L2Port, T>);
impl<T> Deref for ConnectedNode<T> {
    type Target = HashMap<L2Port, T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> L2Switch<T> {
    pub fn new(ip: IpAddrv4, mac: MacAddr, connected_nodes: Vec<ConnectedNode<T>>) -> Self {
        L2Switch {
            ip,
            mac,
            connected_nodes,
            mac_addr_table: MacAddressTable::new(),
        }
    }
    pub fn connect_new_port(&mut self, new_port: ConnectedNode<T>) {
        self.connected_nodes.push(new_port);
    }
    pub fn arp(&self, dist_ip: IpAddrv4) {
        let send_data = b"dist_ip";
        unimplemented!("todo not impl");
    }
    fn send(&self, dist_port: L2Port, data: Vec<u8>) -> Result<(), String> {
        if self
            .connected_nodes
            .iter()
            .any(|node| node.contains_key(&dist_port))
        {
            unimplemented!("todo not impl");
        };
        Err(format!("not connect port: {:?}", dist_port))
    }
    fn get_dist_port(&self, mac_addr: MacAddr) -> Result<&L2Port, String> {
        if let Some(dist_port) = self.mac_addr_table.get_port(mac_addr) {
            Ok(dist_port)
        } else {
            Err(format!("not impl"))
        }
    }
}
