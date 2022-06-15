use super::{macaddr::MacAddr, utils::traits::ToBytes};

pub struct EthernetFlame {
    preamble: [u8; 8],
    src_mac: MacAddr,
    dist_mac: MacAddr,
    e_type: EthType,
}

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
