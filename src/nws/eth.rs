use super::{macaddr::MacAddr, utils::traits::ToBytes};
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct EthernetFlame {
    preamble: Preamble,
    header: EtherHeader,
    payload: Vec<u8>,
    fcs: [u8; 4],
}
impl EthernetFlame {
    pub fn new(src_mac: MacAddr, dist_mac: MacAddr, e_type: EthType, payload: Vec<u8>) -> Self {
        let mut payload = payload;
        let payload_len = payload.len();
        if payload_len > 1500 {
            panic!("plase split data :{:?}", payload);
        }
        if payload_len < 46 {
            let padding_size = 46 - payload_len;
            let padding = vec![0];
            let mut padding = padding.repeat(padding_size);
            payload.append(&mut padding);
        }
        EthernetFlame {
            preamble: Preamble::new(),
            header: EtherHeader::new(src_mac, dist_mac, e_type),
            payload,
            fcs: [0, 0, 0, 0],
        }
    }
}
#[derive(PartialEq, Eq, Debug, Clone)]
struct EtherHeader {
    src_mac: MacAddr,
    dist_mac: MacAddr,
    e_type: EthType,
}
impl EtherHeader {
    fn new(src_mac: MacAddr, dist_mac: MacAddr, e_type: EthType) -> Self {
        EtherHeader {
            src_mac,
            dist_mac,
            e_type,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Preamble([u8; 8]);
impl Preamble {
    fn new() -> Self {
        Preamble([
            0b10101010, 0b10101010, 0b10101010, 0b10101010, 0b10101010, 0b10101010, 0b10101010,
            0b10101011,
        ])
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum EthType {
    IEEE802_3LengthField,
    IPv4,
    Arp,
    RArp,
    IPv6,
    PPPoEDiscoveryStage,
    PPPoESessionStage,
}
impl ToBytes for EthType {
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            EthType::IEEE802_3LengthField => vec![0x00, 0x00],
            EthType::IPv4 => vec![0x08, 0x00],
            EthType::Arp => vec![0x08, 0x06],
            EthType::RArp => vec![0x80, 0x35],
            EthType::IPv6 => vec![0x86, 0xdd],
            EthType::PPPoEDiscoveryStage => vec![0x88, 0x63],
            EthType::PPPoESessionStage => vec![0x88, 0x64],
        }
    }
}

#[cfg(test)]
#[test]
fn eth_new_test() {
    let s_mac = MacAddr::new([100, 100, 100, 100, 100, 100]);
    let d_mac = MacAddr::new([200, 200, 200, 200, 200, 200]);
    let pad = vec![0];
    let mut pad = pad.repeat(44);
    pad.insert(0, 10);
    pad.insert(0, 10);

    assert_eq!(
        EthernetFlame::new(s_mac.clone(), d_mac.clone(), EthType::IPv4, vec![10, 10]),
        EthernetFlame::new(s_mac, d_mac, EthType::IPv4, pad)
    )
}
