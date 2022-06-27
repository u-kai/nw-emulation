pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}
pub trait SendPacket {
    fn send(&self, packet: Vec<u8>) -> ();
}
