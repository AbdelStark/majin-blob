use lazy_static::lazy_static;
use num_bigint::{BigInt, ToBigInt};
use num_traits::Num;
use std::str::FromStr;

// ****************************************************************************
// *  PARAMETERS & CONSTANTS                                                  *
// ****************************************************************************
/// Length of the blob.
pub const BLOB_LEN: usize = 4096;
lazy_static! {
    /// EIP-4844 BLS12-381 modulus.
    ///
    /// As defined in https://eips.ethereum.org/EIPS/eip-4844
    static ref EIP_4844_BLS_MODULUS: BigInt = BigInt::from_str(
        "52435875175126190479447740508185965837690552500527637822603658699938581184513",
    )
    .unwrap();
    /// Generator of the group of evaluation points (EIP-4844 parameter).
    static ref GENERATOR: BigInt = BigInt::from_str(
        "39033254847818212395286706435128746857159659164139250548781411570340225835782",
    )
    .unwrap();
    static ref TWO: BigInt = 2i32.to_bigint().unwrap();
}

/// Recovers the original data from a given blob.
///
/// This function takes a vector of `BigInt` representing the data of a blob and
/// returns the recovered original data as a vector of `BigInt`.
///
/// # Arguments
///
/// * `data` - A vector of `BigInt` representing the blob data.
///
/// # Returns
///
/// A vector of `BigInt` representing the recovered original data.
pub fn blob_recover(data: Vec<BigInt>) -> Vec<BigInt> {
    let xs: Vec<BigInt> = (0..BLOB_LEN)
        .map(|i| {
            let bin = format!("{:012b}", i);
            let bin_rev = bin.chars().rev().collect::<String>();
            GENERATOR.modpow(
                &BigInt::from_str_radix(&bin_rev, 2).unwrap(),
                &EIP_4844_BLS_MODULUS,
            )
        })
        .collect();

    ifft(data, xs, &EIP_4844_BLS_MODULUS)
}

/// Divides two `BigInt` numbers modulo a third `BigInt` number.
///
/// # Arguments
///
/// * `a` - The numerator as a `BigInt`.
/// * `b` - The denominator as a `BigInt`.
/// * `p` - The modulus as a `BigInt`.
///
/// # Returns
///
/// The result of the division modulo `p` as a `BigInt`.
pub fn div_mod(a: BigInt, b: BigInt, p: &BigInt) -> BigInt {
    a * b.modpow(&(p - TWO.clone()), p) % p
}

/// Performs the inverse Fast Fourier Transform on a vector of `BigInt`.
///
/// # Arguments
///
/// * `arr` - A vector of `BigInt` representing the input array.
/// * `xs` - A vector of `BigInt` representing the evaluation points.
/// * `p` - The modulus as a `BigInt`.
///
/// # Returns
///
/// A vector of `BigInt` representing the transformed array.
pub fn ifft(arr: Vec<BigInt>, xs: Vec<BigInt>, p: &BigInt) -> Vec<BigInt> {
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
        let x = &xs[i];

        res0.push(div_mod(a + b, TWO.clone(), p));
        // Handle subtraction to avoid underflow
        let diff = if b > a { p - (b - a) } else { a - b };
        res1.push(div_mod(diff, TWO.clone() * x, p));

        new_xs.push(x.modpow(&TWO.clone(), p));
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
