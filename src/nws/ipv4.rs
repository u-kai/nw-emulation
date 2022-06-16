use std::{fmt::Display, ops::Deref};

#[derive(PartialEq, Eq, Hash)]
pub struct IpAddrv4([u8; 4]);
impl IpAddrv4 {
    pub fn new(addr: [u8; 4]) -> Self {
        IpAddrv4(addr)
    }
}
impl Display for IpAddrv4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}.{}", self.0[0], self.0[1], self.0[2], self.0[3])
    }
}
impl Deref for IpAddrv4 {
    type Target = [u8; 4];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
