use std::collections::HashMap;

use super::{ipv4::IpAddrv4, macaddr::MacAddr};

pub struct ArpTable {
    table: HashMap<IpAddrv4, MacAddr>,
}
