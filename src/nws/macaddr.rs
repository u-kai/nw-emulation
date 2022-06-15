use std::{fmt::Display, ops::Deref};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MacAddr([u8; 6]);
impl MacAddr {
    pub fn new(addr: [u8; 6]) -> Self {
        MacAddr(addr)
    }
    pub fn is_individual(&self) -> bool {
        self.get_i_g_bit() == 0
    }
    pub fn is_group(&self) -> bool {
        println!("{:b}", self.get_i_g_bit());
        self.get_i_g_bit() == 1
    }
    pub fn is_global(&self) -> bool {
        //self[]
        true
    }
    fn get_i_g_bit(&self) -> u8 {
        (self[0]) & 0b00000001
    }
}
impl Display for MacAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:^02x}-{:^02x}-{:^02x}-{:^02x}-{:^02x}-{:^02x}",
            self[0], self[1], self[2], self[3], self[4], self[5],
        )
    }
}

impl Deref for MacAddr {
    type Target = [u8; 6];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[cfg(test)]
mod mac_addr_test {
    use super::MacAddr;

    #[test]
    fn is_group_test() {
        let mac = MacAddr::new([1, 1, 1, 1, 1, 1]);
        assert!(mac.is_group());
        let mac = MacAddr::new([255, 1, 1, 1, 1, 1]);
        assert!(mac.is_group());
        let mac = MacAddr::new([0, 1, 1, 1, 1, 1]);
        assert!(!mac.is_group());
    }
    #[test]
    fn is_individual_test() {
        let mac = MacAddr::new([0, 1, 1, 1, 1, 1]);
        assert!(mac.is_individual());
        let mac = MacAddr::new([254, 1, 1, 1, 1, 1]);
        assert!(mac.is_individual());
        let mac = MacAddr::new([1, 1, 1, 1, 1, 1]);
        assert!(!mac.is_individual());
    }
}
