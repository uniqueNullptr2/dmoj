use std::collections::HashSet;
use std::io::stdin;
use std::str::FromStr;

fn main() {
    let mut s = String::new();
    let stdin = stdin();
    match stdin.read_line(&mut s) {
        Ok(_) => (),
        Err(_) => {
            println!("Couldnt read first line");
        }
    }

    let n = match u32::from_str(s.trim()) {
        Ok(x) => x,
        Err(_) => 0,
    };
    let mut flake = [0u32;6];

    let mut set : HashSet<[u32;6]> = HashSet::new();
    let mut b = false;
    for f in 0..n {
        s.clear();

        match stdin.read_line(&mut s) {
            Ok(_) => (),
            Err(_) => println!("couldnt read line {}", f),
        }

        for (i,e) in s.trim().split_whitespace().enumerate() {
            if i < 6 {
                flake[i] = match u32::from_str(e) {
                    Ok(x) => x,
                    Err(_) => 0,
                };
            }
        }

        b |= add_permutations(&mut set, &mut flake);
    }
    if b {
        println!("Twin snowflakes found.");
    } else {
        println!("No two snowflakes are alike.");
    }
}

fn add_permutations(set: &mut HashSet<[u32;6]>, flake: &mut [u32;6]) -> bool {
    let mut b = set.contains(flake);
    for _ in 0..5 {
        rotate(flake);
        b |= set.contains(flake);
    }
    flip(flake);
    b |= set.contains(flake);
    for _ in 0..5 {
        rotate(flake);
        b |= set.contains(flake);
    }
        set.insert(flake.clone());
    b
}

fn rotate(flake: &mut [u32;6]) {
    let t = flake[0];
    for i in 1..6 {
        flake[i-1] = flake[i];
    }
    flake[5] = t;
}

fn flip(flake: &mut [u32;6]) {
    flake.swap(0, 5);
    flake.swap(1, 4);
    flake.swap(2, 3);
}
