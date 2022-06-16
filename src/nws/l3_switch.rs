use std::collections::HashMap;

use super::{cidr::CIDR, ipv4::IpAddrv4, macaddr::MacAddr};

pub struct L3Switch {
    ip: IpAddrv4,
    mac: MacAddr,
    table: RouteTable,
}
impl L3Switch {
    pub fn new(ip: IpAddrv4, mac: MacAddr) -> Self {
        L3Switch {
            ip,
            mac,
            table: RouteTable::new(),
        }
    }
    pub fn add_route(&mut self, src: Source, next: NextHop) -> &mut Self {
        self.table.add_route(src, next);
        self
    }
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

#[cfg(test)]
mod l3_switch_tests {
    use crate::nws::{cidr::CIDR, ipv4::IpAddrv4, macaddr::MacAddr};

    use super::L3Switch;

    #[test]
    fn next_hop_test() {
        let mut router = L3Switch::new(
            IpAddrv4::new([192, 168, 1, 1]),
            MacAddr::new([0, 0, 0, 0, 0, 0]),
        );
        let local = CIDR::new(IpAddrv4::new([10, 0, 0, 0]), 16);
        //router
        //.add_route(local, )
        //.add_route(IpAddrv4::new([0, 0, 0, 0]), IpAddrv4::new([192, 168, 2, 1]));
    }
}
