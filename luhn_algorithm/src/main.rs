#![allow(unused_variables, dead_code)]

pub fn luhn(cc_number: &str) -> bool {
    //remove spaces and reject if less than two digits
    let mut num = rem_space(cc_number);
    if num.len() < 2 {
        return false;
    }
    //Moving from right to left, double every second digit: for the number 1234, we double 3 and 1. For the number 98765, we double 6 and 8.
    num = double_and_add(num);
    //Sum all the undoubled and doubled digits.
    let mut s: u32 = 0;
    for i in 0..num.len() {
        s += num[i];
    }
    if s % 10 == 0 {
        return true;
    }
    false
}
fn double_and_add(vi: Vec<u32>) -> Vec<u32> {
    let mut v: Vec<u32> = vi;
    v.reverse();
    for i in 0..v.len() {
        if i % 2 != 0 {
            v[i] = v[i] * 2;
        }
    }
    for i in 0..v.len() {
        if v[i] > 9 {
            v[i] = (v[i] % 10) + (v[i] / 10);
        }
    }
    v.reverse();
    println!("{:?}", v);
    v
}
fn rem_space(line: &str) -> Vec<u32> {
    let mut res: Vec<u32> = Vec::new();
    for i in line.chars() {
        if i.is_digit(10) {
            res.push(i.to_digit(10).unwrap())
        }
    }
    // println!("{:?}", res);
    res
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

#[allow(dead_code)]
fn main() {
    luhn("4223 9826 4026 9299");
}
