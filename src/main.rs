use num_bigint::{BigUint, ToBigUint};
use num_traits::Num;
use std::{fs, str::FromStr}; // Import Num trait

const BLOB_LEN: usize = 4096;

fn div_mod(a: BigUint, b: BigUint, p: &BigUint) -> BigUint {
    a * mod_inv(b, p) % p
}

fn mod_inv(value: BigUint, modulus: &BigUint) -> BigUint {
    let two = 2u32.to_biguint().unwrap();
    if modulus > &two {
        value.modpow(&(modulus - &two), modulus)
    } else {
        panic!("Modulus must be greater than 2 for modular inversion.")
    }
}

fn ifft(arr: Vec<BigUint>, xs: Vec<BigUint>, p: &BigUint) -> Vec<BigUint> {
    // Base case: return immediately if the array length is 1
    if arr.len() == 1 {
        return arr;
    }

    let n = arr.len() / 2;
    let mut res0 = Vec::with_capacity(n);
    let mut res1 = Vec::with_capacity(n);
    let mut new_xs = Vec::with_capacity(n);

    for i in (0..2 * n).step_by(2) {
        let a = &arr[i];
        let b = &arr[i + 1];
        let x = &xs[i / 2];

        res0.push(div_mod(a + b, 2u32.to_biguint().unwrap(), p));

        // Handle subtraction to avoid underflow
        let diff = if b > a { p - (b - a) } else { a - b };

        res1.push(div_mod(diff, 2u32.to_biguint().unwrap() * x, p));
        new_xs.push(x.modpow(&2u32.to_biguint().unwrap(), p));
    }

    // Recursive calls
    let merged_res0 = ifft(res0, new_xs.clone(), p);
    let merged_res1 = ifft(res1, new_xs, p);

    // Merging the results
    let mut merged = Vec::with_capacity(arr.len());
    for i in 0..n {
        merged.push(merged_res0[i].clone());
        merged.push(merged_res1[i].clone());
    }
    merged
}

fn main() {
    let eip_4844_bls_modulus = BigUint::from_str(
        "52435875175126190479447740508185965837690552500527637822603658699938581184513",
    )
    .unwrap();

    let z = BigUint::from_str(
        "39033254847818212395286706435128746857159659164139250548781411570340225835782",
    )
    .unwrap();

    let blob_hex =
        fs::read_to_string("./examples/blob/sn_blob_goerli.txt").expect("Failed to read file");
    let blob_hex = blob_hex.trim();

    let data: Vec<BigUint> = (0..BLOB_LEN)
        .map(|i| BigUint::from_str_radix(&blob_hex[i * 64..(i + 1) * 64], 16).unwrap())
        .collect();

    let xs: Vec<BigUint> = (0..BLOB_LEN)
        .map(|i| {
            let bin = format!("{:012b}", i);
            let bin_rev = bin.chars().rev().collect::<String>();
            z.modpow(
                &BigUint::from_str_radix(&bin_rev, 2).unwrap(),
                &eip_4844_bls_modulus,
            )
        })
        .collect();

    let res = ifft(data, xs, &eip_4844_bls_modulus);

    println!("{:?}", res);
}
