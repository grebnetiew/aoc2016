use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines(); // iterator of Result<String>

    // Part A. Triangles by line

    let mut n_possible = 0;

    for line in input {
        let s = line.unwrap();

        let v: Vec<u32> = s.split(char::is_whitespace)
            .filter_map(|subs| match subs.parse() {
                Ok(n) => Some(n),
                Err(_) => None,
            })
            .collect();

        if is_valid_triangle(v[0], v[1], v[2]) {
            n_possible += 1;
        }
    }

    println!("{}", n_possible);
}

fn is_valid_triangle(a: u32, b: u32, c: u32) -> bool {
    a + b > c && b + c > a && c + a > b
}
