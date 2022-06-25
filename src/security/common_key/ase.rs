use super::traits::KeyGen;

pub struct ASE {
    keygen: dyn KeyGen,
}
