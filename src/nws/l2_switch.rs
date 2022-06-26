use super::{
    ipv4::IpAddrv4,
    mac_address_table::{L2Port, MacAddressTable},
    macaddr::MacAddr,
};

pub struct L2Switch {
    ip: IpAddrv4,
    mac: MacAddr,
    mac_addr_table: MacAddressTable,
}

impl L2Switch {
    fn get_dist_port(&self, mac_addr: MacAddr) -> Result<&L2Port, String> {
        if let Some(dist_port) = self.mac_addr_table.get_port(mac_addr) {
            Ok(dist_port)
        } else {
            Err(format!("not impl"))
        }
    }
}
