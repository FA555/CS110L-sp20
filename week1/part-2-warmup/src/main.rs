/* The following exercises were borrowed from Will Crichton's CS 242 Rust lab. */

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::collections::HashSet;

fn main() {
    println!("Hi! Try running \"cargo test\" to run tests.");
}

fn add_n(v: Vec<i32>, n: i32) -> Vec<i32> {
    let mut v = v;
    for x in &mut v {
        *x += n;
    }
    v
}

fn add_n_inplace(v: &mut Vec<i32>, n: i32) {
    for x in v {
        *x += n;
    }
}

fn dedup(v: &mut Vec<i32>) {
    let mut seen = HashSet::new();
    let mut v_ret = Vec::new();
    
    for x in &mut *v {
        if !seen.contains(x) {
            seen.insert(*x);
            v_ret.push(*x);
        }
    }
    *v = v_ret;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_n() {
        let v = vec![1, 2, 3];
        assert_eq!(add_n(v, 2), vec![3, 4, 5]);
    }

    #[test]
    fn test_add_n_inplace() {
        let mut v = vec![1];
        add_n_inplace(&mut v, 2);
        assert_eq!(v, vec![3]);
    }

    #[test]
    fn test_dedup() {
        let mut v = vec![3, 1, 0, 1, 4, 4];
        dedup(&mut v);
        assert_eq!(v, vec![3, 1, 0, 4]);
    }
}
