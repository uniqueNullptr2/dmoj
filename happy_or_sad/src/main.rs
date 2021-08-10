use std::io::stdin;

fn main() {
    let mut input = String::new();

    stdin().read_line(&mut input).unwrap();

    let mut happy = 0;
    let mut sad = 0;

    for ch in input.chars().collect::<Vec<char>>()[..].windows(3) {
        if ch[0] == ':' && ch[1] == '-' && ch[2] == ')' {
            happy += 1;
        } else if ch[0] == ':' && ch[1] == '-' && ch[2] == '(' {
            sad += 1;
        }
    }

    if happy == 0 && sad == 0 {
        println!("none");
    } else if happy > sad {
        println!("happy");
    } else if sad > happy {
        println!("sad");
    } else {
        println!("unsure");
    }
}
