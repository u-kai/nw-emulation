use std::fmt::Display;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MacAddr([u8; 6]);
impl MacAddr {
    pub fn new(addr: [u8; 6]) -> Self {
        MacAddr(addr)
    }
}
impl Display for MacAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:^02x}-{:^02x}-{:^02x}-{:^02x}-{:^02x}-{:^02x}",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5],
        )
    }
}
