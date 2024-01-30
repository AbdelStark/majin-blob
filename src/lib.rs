use num_bigint::{BigUint, ToBigUint};
use num_traits::Num;
use std::str::FromStr;
const BLOB_LEN: usize = 4096;

use lazy_static::lazy_static;

lazy_static! {
    /// EIP-4844 BLS12-381 modulus.
    ///
    /// As defined in https://eips.ethereum.org/EIPS/eip-4844
    static ref EIP_4844_BLS_MODULUS: BigUint = BigUint::from_str(
        "52435875175126190479447740508185965837690552500527637822603658699938581184513",
    )
    .unwrap();
    /// Generator of the group of evaluation points (EIP-4844 parameter).
    static ref GENERATOR: BigUint = BigUint::from_str(
        "39033254847818212395286706435128746857159659164139250548781411570340225835782",
    )
    .unwrap();
}

/// Recovers the original data from a given blob.
///
/// This function takes a vector of `BigUint` representing the data of a blob and
/// returns the recovered original data as a vector of `BigUint`.
///
/// # Arguments
///
/// * `data` - A vector of `BigUint` representing the blob data.
///
/// # Returns
///
/// A vector of `BigUint` representing the recovered original data.
pub fn blob_recover(data: Vec<BigUint>) -> Vec<BigUint> {
    let xs: Vec<BigUint> = (0..BLOB_LEN)
        .map(|i| {
            let bin = format!("{:012b}", i);
            let bin_rev = bin.chars().rev().collect::<String>();
            GENERATOR.modpow(
                &BigUint::from_str_radix(&bin_rev, 2).unwrap(),
                &EIP_4844_BLS_MODULUS,
            )
        })
        .collect();

    ifft(data, xs, &EIP_4844_BLS_MODULUS)
}

/// Divides two `BigUint` numbers modulo a third `BigUint` number.
///
/// # Arguments
///
/// * `a` - The numerator as a `BigUint`.
/// * `b` - The denominator as a `BigUint`.
/// * `p` - The modulus as a `BigUint`.
///
/// # Returns
///
/// The result of the division modulo `p` as a `BigUint`.
pub fn div_mod(a: BigUint, b: BigUint, p: &BigUint) -> BigUint {
    a * mod_inv(b, p) % p
}

/// Calculates the modular inverse of a `BigUint` number modulo another `BigUint` number.
///
/// # Arguments
///
/// * `value` - The `BigUint` number to invert.
/// * `modulus` - The modulus as a `BigUint`.
///
/// # Returns
///
/// The modular inverse of `value` modulo `modulus`.
///
/// # Panics
///
/// Panics if the modulus is less than or equal to 2.
pub fn mod_inv(value: BigUint, modulus: &BigUint) -> BigUint {
    let two = 2u32.to_biguint().unwrap();
    if modulus > &two {
        value.modpow(&(modulus - &two), modulus)
    } else {
        panic!("Modulus must be greater than 2 for modular inversion.")
    }
}

/// Performs the inverse Fast Fourier Transform on a vector of `BigUint`.
///
/// # Arguments
///
/// * `arr` - A vector of `BigUint` representing the input array.
/// * `xs` - A vector of `BigUint` representing the evaluation points.
/// * `p` - The modulus as a `BigUint`.
///
/// # Returns
///
/// A vector of `BigUint` representing the transformed array.
pub fn ifft(arr: Vec<BigUint>, xs: Vec<BigUint>, p: &BigUint) -> Vec<BigUint> {
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
        //res1.push(div_mod(a - b, 2u32.to_biguint().unwrap() * x, p));
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
