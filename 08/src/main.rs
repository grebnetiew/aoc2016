extern crate regex;
use regex::Regex;

use std::io;
use std::io::BufRead;
use std::collections::HashSet;

const SCREENW: usize = 50;
const SCREENH: usize = 6;

fn main() {
    let mut screen = Vec::new();
    let r1 = Regex::new(r"rect (\d+)x(\d+)").unwrap();
    let r2 = Regex::new(r"rotate row y=(\d+) by (\d+)").unwrap();
    let r3 = Regex::new(r"rotate column x=(\d+) by (\d+)").unwrap();

    let stdin = io::stdin();
    for l in stdin.lock().lines() {
        let s = l.unwrap();
        if r1.is_match(&s) {
            let caps = r1.captures(&s).unwrap();
            for i in 0..caps.get(1).unwrap().as_str().parse::<usize>().unwrap() {
                for j in 0..caps.get(2).unwrap().as_str().parse::<usize>().unwrap() {
                    screen.push((i, j));
                }
            }
        } else if r2.is_match(&s) {
            let caps = r2.captures(&s).unwrap();
            for mut p in &mut screen {
                if p.1 == caps.get(1).unwrap().as_str().parse::<usize>().unwrap() {
                    p.0 = (p.0 + caps.get(2).unwrap().as_str().parse::<usize>().unwrap()) % SCREENW;
                }
            }
        } else if r3.is_match(&s) {
            let caps = r3.captures(&s).unwrap();
            for mut p in &mut screen {
                if p.0 == caps.get(1).unwrap().as_str().parse::<usize>().unwrap() {
                    p.1 = (p.1 + caps.get(2).unwrap().as_str().parse::<usize>().unwrap()) % SCREENH;
                }
            }
        }
    }

    let mut amount = 0;
    let mut pixelset = HashSet::new();

    for &p in &screen {
        if pixelset.insert(p) {
            amount += 1;
        }
    }

    println!("{}", amount);

    printscreen(&screen);
}

fn printscreen(&ref screen: &Vec<(usize, usize)>) {
    let mut printer = vec![vec!['▒'; SCREENW]; SCREENH];
    for p in screen {
        printer[p.1][p.0] = '█';
    }
    for l in printer {
        println!("{}", l.into_iter().collect::<String>());
    }
}
