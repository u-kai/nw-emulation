use std::{fmt::Display, u32::MAX};

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
    pub fn cidr_str(&self) -> String {
        let mut str_addr = self.addr.iter().fold(String::new(), |acc, cur| {
            format!("{}{}", acc, CIDR::to_bynary_str(*cur))
        });
        let diff = 32 - self.block;
        for i in 0..diff {
            let remove_index = 32 - (i + 1);
            str_addr.remove(remove_index as usize);
            str_addr.push('0');
        }
        let mut result = String::new();
        let mut start = 0;
        for i in 1..=32 {
            if i % 8 == 0 {
                result = format!(
                    "{}.{}",
                    result,
                    CIDR::bynary_str_to_u8(str_addr.get(start..i).unwrap())
                );
                start = i;
            }
        }
        result.remove(0);
        format!("{}/{}", result, self.block)
    }

    fn to_bynary_str(byte: u8) -> String {
        let mut byte = byte;
        let mut result = String::new();
        while byte > 0 {
            result = format!("{}{}", byte % 2, result);
            byte = byte / 2;
        }
        format!("{:0>8}", result)
    }
    fn bynary_str_to_u8(str: &str) -> String {
        let str = str
            .chars()
            .rev()
            .enumerate()
            .fold(String::new(), |acc, (i, cur)| match cur {
                '1' => format!(
                    "{}",
                    acc.parse::<usize>().unwrap_or(0) + (2_i32.pow(i as u32) as usize)
                ),
                '0' => acc,
                _ => panic!("not num,{}", cur),
            });
        if str.len() == 0 {
            return "0".to_string();
        } else {
            str
        }
    }
}

impl Display for CIDR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.cidr_str())
    }
}

mod cidr_test {
    use super::CIDR;
    use crate::nws::ipv4::IpAddrv4;

    #[test]
    fn new_test() {
        let ipv4 = IpAddrv4::new([192, 168, 1, 2]);
        let cidr = CIDR::new(ipv4, 16);
        assert_eq!(cidr.cidr_str(), "192.168.0.0/16");
    }
}
