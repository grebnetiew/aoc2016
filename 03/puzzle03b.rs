use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines(); // iterator of Result<String>

    // Part B. Triangles by column

    let mut n_possible = 0;

    loop {
        let (s1, s2, s3): (String, String, String);

        if let Some(l) = input.next() {
            s1 = l.unwrap();
        } else {
            break;
        }
        if let Some(l) = input.next() {
            s2 = l.unwrap();
        } else {
            break;
        }
        if let Some(l) = input.next() {
            s3 = l.unwrap();
        } else {
            break;
        }

        let u: Vec<u32> = s1.split(char::is_whitespace)
            .filter_map(|subs| match subs.parse() {
                Ok(n) => Some(n),
                Err(_) => None,
            })
            .collect();

        let v: Vec<u32> = s2.split(char::is_whitespace)
            .filter_map(|subs| match subs.parse() {
                Ok(n) => Some(n),
                Err(_) => None,
            })
            .collect();

        let w: Vec<u32> = s3.split(char::is_whitespace)
            .filter_map(|subs| match subs.parse() {
                Ok(n) => Some(n),
                Err(_) => None,
            })
            .collect();

        if is_valid_triangle(u[0], v[0], w[0]) {
            n_possible += 1;
        }
        if is_valid_triangle(u[1], v[1], w[1]) {
            n_possible += 1;
        }
        if is_valid_triangle(u[2], v[2], w[2]) {
            n_possible += 1;
        }
    }

    println!("{}", n_possible);
}

fn is_valid_triangle(a: u32, b: u32, c: u32) -> bool {
    a + b > c && b + c > a && c + a > b
}
