use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let input = stdin
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap();
    let mut line = input.chars();

    println!("{}", parse_normal(&mut line));
}

fn parse_normal(mut input: &mut Iterator<Item = char>) -> usize {
    let mut c_opt = input.next();
    let mut out = 0;

    while c_opt.is_some() {
        let c = c_opt.unwrap();
        match c {
            '(' => out += parse_compressed(&mut input),
            _ => out += 1,
        }
        c_opt = input.next();
    }

    out
}

fn parse_compressed(input: &mut Iterator<Item = char>) -> usize {
    // (datalength)x(repetitions)

    let mut out = String::new();
    let mut c = input.next().unwrap();
    while c != 'x' {
        out.push(c);
        c = input.next().unwrap();
    }
    let datalength = out.parse::<usize>().unwrap();

    out = String::new();
    c = input.next().unwrap();
    while c != ')' {
        out.push(c);
        c = input.next().unwrap();
    }
    let repetitions = out.parse::<usize>().unwrap();

    out = String::new();
    for _ in 0..datalength {
        out.push(input.next().unwrap());
    }

    repetitions * parse_normal(&mut out.chars())
}