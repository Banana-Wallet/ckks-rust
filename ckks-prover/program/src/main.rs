//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);


use approx::assert_relative_eq;

use num_bigint::ToBigInt;
use rlwe::encode;
use rlwe::encoder;
use rlwe::*;
use tfhe_test::run;

use algebra::crt::Crt;

pub fn main() {
    // NOTE: values of n larger than 186 will overflow the u128 type,
    // resulting in output that doesn't match fibonacci sequence.
    // However, the resulting proof will still be valid!
    let n = sp1_zkvm::io::read::<u32>();
    run();


    // // let n = 8;

    // let encoder = encoder::CKKSEncoder::new(n as usize * 2);

    // let x = [0.5, 0.3, 0.78, 0.88];
    // let plainx = encode(&x, 1usize << 30, &encoder);

    // let q = 1.to_bigint().unwrap() << 600;

    // let encoder = encoder::CKKSEncoder::new(n as usize * 2);

    // Create a keypair 100 times
    // for _ in 0..100 {
        // Create key pair
        // let key = Rwle::keygen(&q, n as usize, n as usize);

        // // Create our data with a single datapoint
        // let x = 0.02_64;
        // let data = vec![x; (n / 2) as usize];

        // let plain = encode(data.as_slice(), 1usize << 30, &encoder);

        // // Encrypt our data using keypair
        // let cipher1 = encrypt(key.public(), &q, &plain);

        // // Decrypt our data
        // let out = decrypt(key.private(), cipher1);

        // let decode = decode(out, &encoder);

        // assert_relative_eq!(x, decode[0].re, epsilon = 1e-4);
    // }
    // sp1_zkvm::io::write(&a);
    // sp1_zkvm::io::write(&b);

    // let n = 8;

    // let q = 1.to_bigint().unwrap() << 600;

    // let key = Rwle::keygen(&q, n as usize, n as usize);
    // let encoder = encoder::CKKSEncoder::new(n as usize * 2);

    // let x = [0.05, 0.1, 1.0, 0.005];
    // let y = [0.1, 0.02, 0.5, 0.3];

    // let plainx = encode(&x, 1usize << 30, &encoder);
    // let plainy = encode(&y, 1usize << 30, &encoder);

    // let cipherx = encrypt(key.public(), &q, &plainx);
    // let ciphery = encrypt(key.public(), &q, &plainy);

    // let cipherz = &cipherx + &ciphery;

    // let plainz = decrypt(key.private(), cipherz);

    // let z = decode(plainz, &encoder);

    // let expected_z: Vec<f64> = x.iter().zip(&y).map(|(a, b)| a + b).collect();

    // for (&x, y) in expected_z.iter().zip(z) {
    //     assert_relative_eq!(x, y.re, epsilon = 1e-4)
    // }
}
