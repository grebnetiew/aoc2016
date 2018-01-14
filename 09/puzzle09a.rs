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

    let mut c_opt = line.next();
    let mut out = String::new();

    while c_opt.is_some() {
        let c = c_opt.unwrap();
        match c {
            '(' => out += &parse_compressed(&mut line),
            _ => out.push(c),
        }
        c_opt = line.next();
    }

    println!("{}", out.len());
}

fn parse_compressed(ref mut input: &mut Iterator<Item = char>) -> String {
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

    out.repeat(repetitions)
}