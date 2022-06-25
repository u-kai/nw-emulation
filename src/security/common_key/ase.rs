use super::key::KeyGen;

pub struct ASE<T> {
    keygen: Box<dyn KeyGen<T>>,
}
