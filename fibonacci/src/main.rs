use std::io::stdin;
use std::str::FromStr;

fn main() {
    let mut s = String::new();
    let stdin = stdin();
    stdin.read_line(&mut s).unwrap();
    let u = u64::from_str(s.trim()).unwrap();
    let f = fast_double(u);
    println!("{}", f.0);
}
//1000000000000000
fn fast_double(u: u64) -> (i64, i64) {
    if u == 0 {
        (0, 0)
    } else if u == 1 {
        (1, 1)
    } else if u <= 5 {
        let mut n1 = 0;
        let mut n2 = 1;
        for _ in 0..u {
            let t = ((n1 % 1000000007) + (n2 % 1000000007)) % 1000000007;
            n1 = n2;
            n2 = t;
        }
        (n1, n2)
    } else {
        if u % 2 == 0 {
            let (k, k1) = fast_double(u / 2);
            let k2 = fast_k2(k, k1);
            let k21 = (k1.pow(2) + k.pow(2)) % 1000000007;
            (k2, k21)
        } else {
            let (k, k1) = fast_double(u - 1);
            let k2 = ((k) + (k1)) % 1000000007;
            (k1, k2)
        }
    }
}

fn fast_k2(mut k: i64, mut k1: i64) -> i64 {
    k = k % 1000000007;
    k1 = k1 % 1000000007;
    let mut f = (2 * k1 as i64 - k as i64) % 1000000007;
    if f < 0 {
        f = 1000000007 + f;
    }
    (k * f) % 1000000007
}
