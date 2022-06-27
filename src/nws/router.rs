use std::collections::HashMap;

use super::{cidr::CIDR, ipv4::IpAddrv4, macaddr::MacAddr};

pub struct Router {
    ports: Vec<RouterPort>,
    table: RouteTable,
}
struct RouterPort {
    ip: IpAddrv4,
    mac: MacAddr,
}

type Source = CIDR;
type NextHop = IpAddrv4;
struct RouteTable {
    table: HashMap<Source, NextHop>,
}

impl RouteTable {
    pub fn new() -> Self {
        RouteTable {
            table: HashMap::new(),
        }
    }
    pub fn add_route(&mut self, src: Source, next: NextHop) -> &mut Self {
        self.table.insert(src, next);
        self
    }
}
