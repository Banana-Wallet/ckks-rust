use rlwe::*;

#[test]
fn encrypt_decrypt() {
    let n = 2_i32.pow(4);
    let q = 2_i32.pow(15);
    let t = 2_i32.pow(8);

    // Create a keypair 100 times
    for _ in 0..100 {
        // Create key pair
        let key = Rwle::keygen(q, n as usize, n as usize);

        // Create our data with a single datapoint
        let x = 73_i32;
        let mut data = vec![0; n as usize];
        data[0] = x.rem_euclid(t);

        // Encrypt our data using keypair
        let cipher1 = encrypt(&key.public(), n as usize, q, t, n as usize, &data);

        // Decrypt our data
        let out = decrypt(&key.private(), q, t, cipher1);

        assert_eq!(x, out.coef[0]);
    }
}

#[test]
fn add() {
    let n = 2_i32.pow(4);
    let q = 2_i32.pow(15);
    let t = 2_i32.pow(8);

    let key = Rwle::keygen(q, n as usize, n as usize);

    let x = 73_i32;
    let y = 30_i32;

    let datax = vec![x; n as usize];
    let datay = vec![y; n as usize];

    let cipherx = encrypt(&key.public(), n as usize, q, t, n as usize, &datax);
    let ciphery = encrypt(&key.public(), n as usize, q, t, n as usize, &datay);

    let cipherz = &cipherx + &ciphery;

    let z = decrypt(&key.private(), q, t, cipherz);

    let expected_z: Vec<i32> = datax.iter().zip(&datay).map(|(a, b)| a + b).collect();

    assert_eq!(expected_z, z.coef);
}

#[test]
fn mul() {
    let n = 2_i32.pow(4);
    let q = 2_i32.pow(15);
    let t = 2_i32.pow(8);

    let key = Rwle::keygen(q, n as usize, n as usize);

    let x = 2_i32;
    let y = 2_i32;

    let datax = vec![x; n as usize];
    let datay = vec![y; n as usize];

    let cipherx = encrypt(&key.public(), n as usize, q, t, n as usize, &datax);
    let ciphery = encrypt(&key.public(), n as usize, q, t, n as usize, &datay);

    let cipherz = &cipherx * &ciphery;

    let z = decrypt(&key.private(), q, t, cipherz);
    println!("{:?}", z);

    let expected_z: Vec<i32> = datax.iter().zip(&datay).map(|(a, b)| a + b).collect();

    //assert_eq!(expected_z, z.coef);
}
