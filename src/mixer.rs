use crate::round::{Newver, M4};

fn mul2(a: u8) -> u8 {
    if a >> 7 == 1 {
        return ((a << 1) & 0xff) ^ 0x1b;
    } else {
        return (a << 1) & 0xff;
    }
}

fn exmul(a: u8, x: u8) -> u8 {
    if x == 2 {
        return mul2(a);
    } else if x == 3 {
        return mul2(a) ^ a;
    }
    a
}

const MD: M4 = [[2, 3, 1, 1], [1, 2, 3, 1], [1, 1, 2, 3], [3, 1, 1, 2]];
fn mixeral(a: &M4) -> M4 {
    let mut r: M4 = M4::new();
    for i in 0..4 {
        for j in 0..4 {
            r[j][i] = exmul(a[0][i], MD[j][0])
                ^ exmul(a[1][i], MD[j][1])
                ^ exmul(a[2][i], MD[j][2])
                ^ exmul(a[3][i], MD[j][3]);
        }
    }

    r
}

pub fn mix_main() {
    let m: M4 = [
        [0xd6, 0x30, 0xf7, 0xca],
        [0x12, 0x43, 0x6a, 0x05],
        [0x96, 0x3f, 0x5c, 0xf3],
        [0x9a, 0x81, 0x1a, 0x25],
    ];

    let r = mixeral(&m);
    r.println();
}
