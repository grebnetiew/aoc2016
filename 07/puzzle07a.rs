use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let snoopables = stdin
        .lock()
        .lines()
        .map(|l| can_snoop(l.unwrap().as_bytes())) // ignore unicode
        .filter(|b| *b)
        .count();
    println!("{}", snoopables);
}

fn can_snoop(s: &[u8]) -> bool {
    let mut found_abba = false;
    let mut in_brackets = false;
    for i in 0..s.len() {
        if s[i] == b'[' {
            in_brackets = true;
        } else if s[i] == b']' {
            in_brackets = false;
        } else if i >= 3 && s[i] == s[i - 3] && s[i - 1] == s[i - 2] && s[i] != s[i - 1] {
            // This is an abba
            if in_brackets {
                return false;
            }
            found_abba = true;
        }
    }
    found_abba
}
