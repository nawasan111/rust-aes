pub type M4 = [[u8; 4]; 4];

pub trait Newver {
    fn new() -> Self;
    fn println(&self);
}

impl Newver for M4 {
    fn println(&self) {
        for i in 0..4 {
            println!(
                "{:02x} {:02x} {:02x} {:02x}",
                self[i][0], self[i][1], self[i][2], self[i][3],
            )
        }
    }
    fn new() -> Self {
        return [[0u8; 4]; 4];
    }
}

pub const A1: M4 = [
    [0x4a, 0x54, 0x49, 0x25],
    [0x28, 0x28, 0xf8, 0x47],
    [0x32, 0x72, 0x47, 0x86],
    [0x46, 0xd6, 0x36, 0x23],
];

pub const A2: M4 = [
    [0x55, 0xf4, 0x90, 0x69],
    [0x44, 0x86, 0x9a, 0x27],
    [0x33, 0x47, 0x6c, 0x14],
    [0x22, 0x11, 0x72, 0x88],
];

pub fn multixor(a: M4, b: M4) -> M4 {
    let mut r: M4 = [[0; 4]; 4];
    for i in 0..4 {
        r[0][i] = a[0][i] ^ b[0][i];
        r[1][i] = a[1][i] ^ b[1][i];
        r[2][i] = a[2][i] ^ b[2][i];
        r[3][i] = a[3][i] ^ b[3][i];
    }
    r
}

pub fn round_main() {
    let r = multixor(A1, A2);
    r.println();
}
