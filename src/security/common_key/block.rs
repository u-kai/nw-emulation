struct BlockEncription<T: SubKey> {
    sub_keys: Vec<T>,
    enc: Enc,
    dec: Dec,
    msg: Vec<u8>,
}
//impl BlockEncription {
//pub fn new(secret_key: Vec<u8>, msg: &str) -> Self {}
//}
trait KeySchedule<T: SubKey> {
    fn sub_key_gen(&self) -> Vec<T>;
}
trait SubKey {
    fn round_func(&self, data: u8) -> u8;
}
struct Enc;

struct Dec;
struct Feistel<T: SubKey> {
    round: usize,
    sub_keys: Vec<T>,
    ls: Vec<u8>,
    rs: Vec<u8>,
}
impl<T: SubKey> Feistel<T> {
    fn new(round: usize, msg: Vec<u8>, sub_keys: Vec<T>) -> Self {
        let len = msg.len();
        let ls = msg.iter().take(len / 2).map(|u| *u).collect();
        let rs = msg.iter().skip(len / 2).map(|u| *u).collect();
        Feistel {
            round,
            rs,
            ls,
            sub_keys,
        }
    }
    fn enc<F>(&mut self) -> Vec<u8> {
        let mut i = 1;
        while i < self.round {
            *self.ls.get_mut(i).unwrap() = *self.rs.get(i - 1).unwrap();
            *self.rs.get_mut(i).unwrap() = *self.ls.get(i - 1).unwrap()
                ^ self
                    .sub_keys
                    .get(i)
                    .unwrap()
                    .round_func(*self.rs.get(i - 1).unwrap());
            i += 1;
        }
        let mut rs = self.rs.drain(..).collect::<Vec<_>>();
        let mut ls = self.ls.drain(..).collect::<Vec<_>>();
        rs.append(&mut ls);
        rs
    }
}

//#[test]
//fn enc_test() {
//let b = BlockEncription {
//key: vec![0],
//msg: vec![1, 0, 1],
//};
//assert_eq!(b.enc(), vec![]);
//}
