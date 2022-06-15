use std::fmt::Display;

pub struct IpAddrv4([i8; 4]);
impl IpAddrv4 {
    pub fn new(addr: [i8; 4]) -> Self {
        IpAddrv4(addr)
    }
}
impl Display for IpAddrv4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}.{}", self.0[0], self.0[1], self.0[2], self.0[3])
    }
}
