pub use polyr::{polynomial, Polynomial};
#[test]
fn mod_cyc() {
    let mut poly = polynomial![0, 77, 7, 11, 12, 1];
    poly.mod_cyc(4);
    assert_eq!(polynomial![-12, 76, 7, 11], poly)
}

#[test]
fn add() {
    let a = polynomial![7, 0, 1, 1];
    let b = polynomial![0, 11, 1];
    let c = a + b;
    assert_eq!(polynomial![7, 11, 2, 1], c);
}

#[test]
fn mul() {
    let a = polynomial![7, 0, 1, 1];
    let b = polynomial![0, 11, 1];
    let c = a * b;
    assert_eq!(polynomial![0, 77, 7, 11, 12, 1], c);
}

#[test]
fn mul_mod_cyc() {
    let a = polynomial![7, 0, 1, 1];
    let b = polynomial![0, 11, 1];
    let mut c = a * b;
    c.mod_cyc(4);
    assert_eq!(polynomial![-12, 76, 7, 11], c)
}

#[test]
fn add_field_mod_cyc() {
    let a = polynomial![7, 0, 1, 1];
    let b = polynomial![0, 11, 1];
    let mut c = a + b;
    c.mod_cyc(4);
    c.rem_euclid(5);
    assert_eq!(polynomial![2, 1, 2, 1], c)
}

#[test]
fn mul_field_mod_cyc() {
    let a = polynomial![7, 0, 1, 1];
    let b = polynomial![0, 11, 1];
    let mut c = a * b;
    c.mod_cyc(4);
    c.rem_euclid(5);
    assert_eq!(polynomial![3, 1, 2, 1], c)
}

#[test]
fn rlwe() {
    let n = 2 * 2; // n = 2^k = len(a)
    let q = 13; // q = 1 mod 2n

    let a = polynomial![10, 11, 1, 4];
    let s = polynomial![11, 11, 9, 6];
    let e = polynomial![1, 1, -1, 0];

    let mut mul = a * s;
    mul.rem_euclid(q);
    mul.mod_cyc(n);

    let mut res = mul + e;
    res.rem_euclid(q);
    res.mod_cyc(n);

    assert_eq!(polynomial![5, 8, 2, 6], res);
}
