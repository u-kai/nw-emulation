use crate::security::gen_rand_bytes::gen_rand_bytes;

pub struct Vernam {
    msg: Vec<u8>,
    key: Vec<u8>,
}

impl Vernam {
    fn new(msg: &str) -> Self {
        let msg = msg.bytes().map(|byte| byte).collect::<Vec<u8>>();
        let len = msg.len();
        let key = gen_rand_bytes(len);
        Vernam { msg, key }
    }
    fn enc(&self) -> Vec<u8> {
        self.msg
            .iter()
            .enumerate()
            .map(|(i, byte)| byte ^ self.key.get(i).unwrap())
            .collect()
    }
    fn dec(&self) -> Vec<u8> {
        self.enc()
            .iter()
            .enumerate()
            .map(|(i, byte)| byte ^ self.key.get(i).unwrap())
            .collect()
    }
}

#[test]
fn enc_test() {
    let v = Vernam {
        msg: vec![0],
        key: vec![1],
    };
    assert_eq!(v.enc(), vec![1]);
    let v = Vernam {
        msg: vec![0b0, 0b0, 0b1],
        key: vec![0b1, 0b0, 0b1],
    };
    assert_eq!(v.enc(), vec![1, 0, 0]);
    let v = Vernam {
        msg: vec![0b0, 0b0, 0b1, 0b1],
        key: vec![0b1, 0b0, 0b1, 0b0],
    };
    assert_eq!(v.enc(), vec![1, 0, 0, 1]);
}

#[test]
fn dec_test() {
    let v = Vernam {
        msg: vec![0],
        key: vec![1],
    };
    assert_eq!(v.dec(), vec![0]);
    let v = Vernam {
        msg: vec![0b0, 0b0, 0b1],
        key: vec![0b1, 0b0, 0b1],
    };
    assert_eq!(v.dec(), vec![0, 0, 1]);
    let v = Vernam {
        msg: vec![0b0, 0b0, 0b1, 0b1],
        key: vec![0b1, 0b0, 0b1, 0b0],
    };
    assert_eq!(v.dec(), vec![0, 0, 1, 1]);
}

#[test]
fn new_test() {
    let v = Vernam::new("Hello World");
    assert_eq!(v.dec(), v.msg);
    println!("{:?}", v.enc());
}
