use std::u8::MAX;

use super::{
    eth::{EthType, EthernetFlame},
    ipv4::IpAddrv4,
    macaddr::MacAddr,
};

pub struct Arp {
    dist_mac: ArpDist,
    src_mac: MacAddr,
    src_ip: IpAddrv4,
    dist_ip: IpAddrv4,
    e_type: EthType,
}

impl Arp {
    pub fn new(src_mac: MacAddr, src_ip: IpAddrv4, dist_ip: IpAddrv4) -> Self {
        Arp {
            dist_mac: ArpDist::new(),
            src_mac,
            src_ip,
            dist_ip,
            e_type: EthType::Arp,
        }
    }
}

struct ArpDist(MacAddr);
impl ArpDist {
    fn new() -> Self {
        ArpDist(MacAddr::new([MAX; 6]))
    }
}
