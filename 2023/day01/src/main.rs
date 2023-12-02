#![feature(portable_simd)]

use std::simd::{u8x64, Mask, SimdPartialOrd, Simd, SimdUint};
use rayon::str::ParallelString;
use rayon::*;

fn pad_out_and_convert_line(line: &[u8]) -> u8x64 {
    const LEN: usize = 64;

    if line.len() == LEN {
        return u8x64::from_slice(line);
    }
    
    let mut ret: [u8; LEN] = [0; LEN];
    
    if line.len() < LEN {
        ret[..line.len()].copy_from_slice(line);
        return u8x64::from_slice(&ret);
    }

    println!("Line had more than 64 bytes! Line len: {}", line.len());
    panic!();
}

fn convert_char_digits_to_uint(simd_value: u8x64) -> u8x64 {
    // TODO: Find a way to have these as constants or w/e
    let ZERO_ASCII: Simd<u8, 64> = u8x64::splat(b'0');
    let NINE: Simd<u8, 64> = u8x64::splat(9);
    let NULL: Simd<u8, 64> = u8x64::splat(u8::MAX);

    // print!("\n\t -- orig: [");
    // for val in simd_value.as_array() {
    //     print!("{}, ", val);
    // }
    // print!("]\n\t -- vals: [");

    let val = simd_value - ZERO_ASCII;
    let mask = val.simd_gt(NINE);

    // for val in val.as_array() {
    //     print!("{}, ", val);
    // }
    // print!("]\n\t -- rets: [");

    let res = mask.select(NULL, val);

    // for val in res.as_array() {
    //     print!("{}, ", val);
    // }
    // print!("]\n\t == ");

    // for val in res.as_array() {
    //     if *val < 10 {
    //         print!("{}, ", val);
    //     }
    // }

    // print!("\n");

    res
}

fn vec_to_num(simd_val: u8x64) -> u32 {
    let mut iter = simd_val.as_array().iter().filter(|el| **el < 10);
    let mut iter2 = iter.clone().rev();
    let first = iter.next().unwrap();
    let last = iter2.next().unwrap();

    10u32 * *first as u32 + *last as u32
}

fn calc_solution_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| pad_out_and_convert_line(line.to_ascii_lowercase().as_bytes()))
        .map(|bytes| convert_char_digits_to_uint(bytes))
        .map(|bytes| vec_to_num(bytes))
        .sum()
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", calc_solution_1(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pad_out_and_convert_line_of_len_64() {
        let line_exact = [b'a'; 64];
        let result = pad_out_and_convert_line(&line_exact);
        assert_eq!(result, u8x64::from_slice(&line_exact));
    }

    #[test]
    fn test_pad_out_and_convert_line_of_len_30() {
        let line_short = [b'b'; 30];
        let mut expected = [b'b'; 64];
        expected[30..].fill(0);
        let result = pad_out_and_convert_line(&line_short);
        assert_eq!(result, u8x64::from_slice(&expected));
    }

    #[test]
    fn test_pad_out_and_convert_line_of_len_65() {
        let line_long = [b'c'; 65];
        let result = std::panic::catch_unwind(|| pad_out_and_convert_line(&line_long));
        assert!(result.is_err());
    }

    #[test]
    fn test_digit_conversion() {
        let input = ['5' as u8; 64];
        let expected_output = [5 as u8; 64];

        let raw_output = convert_char_digits_to_uint(u8x64::from_slice(&input));
        let output = raw_output.as_array();

        assert_eq!(expected_output, *output);
    }

    #[test]
    fn test_first_half() {
        let input = include_str!("test_input.txt");
        assert_eq!(142, calc_solution_1(input));
    }
}
