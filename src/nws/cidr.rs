use std::fmt::Display;

use super::ipv4::IpAddrv4;

#[derive(PartialEq, Eq, Hash)]
pub struct CIDR {
    addr: IpAddrv4,
    block: u8,
}

impl CIDR {
    pub fn new(addr: IpAddrv4, block: u8) -> Self {
        CIDR { addr, block }
    }
    pub fn is_local(&self, addr: IpAddrv4) -> bool {
        true
    }
}

impl Display for CIDR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}/{}",
            self.addr[0], self.addr[1], self.addr[2], self.addr[3], self.block,
        )
    }
}
