use std::io;
use std::io::BufRead;
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();

    let mut line = input.next();
    let linestr = line.unwrap().unwrap();
    let mut frequencies = vec![HashMap::new(); linestr.len()];
    line = Some(Ok(linestr));

    while line.is_some() {
        for (i, c) in line.unwrap().unwrap().char_indices() {
            let f = match frequencies[i].get(&c) {
                Some(n) => n + 1,
                None => 1u32,
            };
            frequencies[i].insert(c, f);
        }

        line = input.next();
    }

    for h in &frequencies {
        print!("{}", find_largest_key(h));
    }
    println!();

    for h in &frequencies {
        print!("{}", find_smallest_key(h));
    }
    println!();
}

fn find_largest_key(h: &HashMap<char, u32>) -> char {
    let mut maximum = 0;
    let mut best_key = ' ';
    for (&key, &val) in h {
        if val > maximum {
            maximum = val;
            best_key = key;
        }
    }
    best_key
}

fn find_smallest_key(h: &HashMap<char, u32>) -> char {
    let mut minimum = std::u32::MAX;
    let mut best_key = ' ';
    for (&key, &val) in h {
        if val < minimum {
            minimum = val;
            best_key = key;
        }
    }
    best_key
}
