pub struct Vernam {
    msg: Vec<u8>,
    key: Vec<u8>,
}

impl Vernam {
    fn enc(&self) -> Vec<u8> {
        self.msg
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
