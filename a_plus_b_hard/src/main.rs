fn main() {
    let stdin = stdin();
    let mut s = String::new();
    stdin.read_line(&mut s).unwrap();
    let u = u32::from_str(s.trim()).unwrap();

    for _ in 0..u {
        s.clear();
        stdin.read_line(&mut s).unwrap();
        let mut t = 0;
        for n in s.trim().split_whitespace() {
            t += i32::from_str(n).unwrap();
        }
        println!("{}", t);
    }
}
