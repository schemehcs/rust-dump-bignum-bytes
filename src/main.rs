use std::process::exit;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    /// input number
    #[arg(value_name = "num")]
    num: String,
}

fn decimal_vec_div2(decimal: &[u8]) -> (Vec<u8>, u8) {
    if decimal.is_empty() {
        (vec![0], 0)
    } else {
        let mut res = Vec::<u8>::new();
        let mut r = 0;
        for digit in decimal {
            let cur = r*10 + *digit;
            let q = cur / 2;
            r = cur % 2;
            if res.is_empty() && q == 0 {
                continue;
            }
            res.push(q);
        }
        if res.is_empty() {
            res.push(0);
        }
        (res, r)
    }
}

fn is_zero(v: &[u8]) -> bool {
    v.len() == 1 && v[0] == 0
}

fn decimal_vec_to_binary_vec(decimal: &[u8]) -> Vec<u8> {
    if is_zero(decimal) {
        return vec![0];
    }
    let mut res = Vec::<u8>::new();
    let mut owned = Vec::new();
    let mut rm;
    owned.extend_from_slice(decimal);
    while !is_zero(&owned) {
        (owned, rm) = decimal_vec_div2(&owned);
        res.push(rm);
    }
    res.reverse();
    res
}

fn bits_to_bytes(bits: &[u8]) -> Vec<Vec<u8>>{
    let mut bytes = Vec::new();
    let md = bits.len() % 8;
    if md != 0 {
        let mut first_vec = Vec::new();
        for _ in 0..8-md {
            first_vec.push(0);
        }
        for i in 0..md {
            first_vec.push(bits[i]);
        }
        bytes.push(first_vec);
    }

    let mut tmp_vec = vec![];
    for j in md..bits.len() {
        tmp_vec.push(bits[j]);
        if tmp_vec.len() == 8 {
            bytes.push(tmp_vec);
            tmp_vec = vec![];
        }
    }
    bytes
}

fn dump_bytes(bytes: &[Vec<u8>]) {
    println!("0 1 2 3 4 5 6 7");
    println!("---------------");
    let mut n = (bytes.len() - 1) as isize;
    for byte in bytes {

        for bit in byte {
            print!("{} ", bit);
        }
        println!("|{}", n);
        n-=1;
    }
}

fn main() {
    let cli = Cli::parse();
    let u8_vec: Vec<u8> = cli.num.chars()
        .skip_while(|c| *c == '0')
        .map(to_u8)
        .collect();
    if u8_vec.is_empty() {
        println!("0");
        exit(0);
    }
    let bits = decimal_vec_to_binary_vec(&u8_vec);
    let bytes = bits_to_bytes(&bits);
    dump_bytes(&bytes);
}

fn to_u8(c: char) -> u8 {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => panic!("invalid character {}", c),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_div2_works() {
        let v  = vec![1_u8,2,3,4,5];
        let (rv, rm) = decimal_vec_div2(&v);
        assert_eq!(rv, vec![6, 1, 7, 2], "invalid div result");
        assert_eq!(rm, 1, "invalid div remainder");
    }

    #[test]
    fn vector_div2_0() {
        let (rv, rm) = decimal_vec_div2(&vec![]);
        assert_eq!(rv, vec![0]);
        assert_eq!(rm, 0);
    }

    #[test]
    fn vector_div2_1() {
        let (rv, rm) = decimal_vec_div2(&vec![1]);
        assert_eq!(rv, vec![0]);
        assert_eq!(rm, 1);
    }

    #[test]
    fn vector_div2_other() {
        let (rv, rm) = decimal_vec_div2(&vec![1, 0, 1]);
        assert_eq!(rv, vec![5, 0]);
        assert_eq!(rm, 1);
    }

    #[test]
    fn decimal_vec_to_binary_vec_works() {
        let res_0 = decimal_vec_to_binary_vec(&[0]);
        assert_eq!(res_0, vec![0], "0 failed");
        let res_1 = decimal_vec_to_binary_vec(&[1]);
        assert_eq!(res_1, vec![1], "1 failed");
        let res_2 = decimal_vec_to_binary_vec(&[2]);
        assert_eq!(res_2, vec![1, 0], "2 failed");
        let res_8 = decimal_vec_to_binary_vec(&[8]);
        assert_eq!(res_8, vec![1, 0, 0, 0], "8 failed");
    }
}