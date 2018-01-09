use std::io;
use std::io::BufRead;
use std::collections::HashMap;
use std::char;

fn main() {
    let stdin = io::stdin();
    let input = stdin
        .lock()
        .lines()
        .filter_map(|s| line_split(s.ok()?))
        .filter(|t| checksum(&t.0) == t.2);

    let mut part_a = 0u32;
    let mut part_b = (0u32, String::new());

    for (name, number, _) in input {
        part_a += number;
        let realname = decrypt(name, number);
        if realname.find("north").is_some() {
            part_b = (number, realname);
        }
    }
    println!("{:?}", part_a);
    println!("{:?}", part_b);
}

fn line_split(s: String) -> Option<(String, u32, String)> {
    let (pre, suf) = s.split_at(s.find('[')?);
    let cs = suf[1..6].to_owned();

    let (name, snum) = pre.split_at(pre.rfind('-')? + 1);
    let number: u32 = snum.parse().ok()?;

    Some((name.to_owned(), number, cs))
}

fn checksum(ref name: &String) -> String {
    let mut frequency = HashMap::new();
    let mut maximum = 0;

    for c in name.chars().filter(|c| c.is_alphabetic()) {
        let f = match frequency.get(&c) {
            Some(n) => n + 1,
            None => 1u32,
        };
        frequency.insert(c, f);
        if f > maximum {
            maximum = f;
        }
    }

    let mut cs = String::new();
    for n in (1..maximum + 1).rev() {
        let mut checkpart = Vec::new();
        for (&c, &amt) in &frequency {
            if amt == n {
                checkpart.push(c);
            }
        }
        checkpart.sort_unstable();
        cs += &checkpart.iter().cloned().collect::<String>();
    }

    cs.truncate(5);
    cs
}

fn decrypt(name: String, number: u32) -> String {
    let a = b'a' as u32;
    name.chars()
        .map(|c| match c {
            '-' => ' ',
            _ => char::from_u32((((c as u32 - a) + number) % 26) + a).unwrap(),
        })
        .collect()
}
