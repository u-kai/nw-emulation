use super::{ipv4::IpAddrv4, mac_address_table::MacAddressTable, macaddr::MacAddr};

pub struct L2Switch {
    ip: IpAddrv4,
    mac: MacAddr,
    mac_addr_table: MacAddressTable,
}
