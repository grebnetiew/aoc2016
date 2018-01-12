use std::io;
use std::io::BufRead;
use std::collections::HashSet;

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
    let mut aba_in_brackets = HashSet::new();
    let mut aba_out_brackets = HashSet::new();
    let mut in_brackets = false;
    for i in 0..s.len() {
        if s[i] == b'[' {
            in_brackets = true;
        } else if s[i] == b']' {
            in_brackets = false;
        } else if i >= 2 && s[i] == s[i - 2] && s[i] != s[i - 1] {
            // This is an aba
            if in_brackets {
                aba_in_brackets.insert((s[i - 1], s[i]));
            } else {
                aba_out_brackets.insert((s[i], s[i - 1]));
            }
        }
    }
    aba_in_brackets
        .intersection(&aba_out_brackets)
        .next()
        .is_some()
}
