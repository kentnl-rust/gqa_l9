extern crate argon2rs;
use argon2rs::u64x2::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let t: u64 = args[1].parse().unwrap();

    let k = u64x2(t + 0, t + 1);
    let m = u64x2(t * 6, t * 24);
    /*let j = u64x2(8, 9);
    let rv = j + k + j.lower_mult(k) * u64x2(2, 2);
    let rv2 = m ^ rv;
    let rv3 = rv2.rotate_right(32);
    println!("{:?} {:?} {:?} {:?} {:?} {:?}", j, k, m, rv, rv2, rv3);*/
    println!("{:?}", k.cross_swap(m));
}
