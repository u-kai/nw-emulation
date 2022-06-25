pub trait KeyGen<T> {
    fn gen(&self, k: T) -> Key;
}

pub struct Key;

pub trait Encript {
    fn enc(&self, msg: &str, key: Key) -> EncriptedData;
}
pub trait Decript {
    fn dec(&self, enc: &EncriptedData, key: Key) -> String;
}
pub struct EncriptedData;
