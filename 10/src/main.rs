extern crate regex;
use regex::Regex;

use std::io;
use std::io::BufRead;

#[derive(Copy, Clone)]
  // Automatically provide a copy constructor that just copies the bytes of my struct
  // and a Clone constructor that (here) does the same
  // (Normally, only a move constructor is present)
  // (If you would like special behavior, provide a custom Clone and delete Copy)
struct Bot {
    contents1: Option<i32>,
    contents2: Option<i32>,
    lower: Option<Destination>,
    upper: Option<Destination>,
}

#[derive(Copy, Clone)]
enum Destination {
    BotNumber(usize),
    OutputNumber(usize),
}

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();

    let mut bots: [Bot; 220] = [Bot{contents1: None, contents2: None, lower: None, upper: None}; 220]; // I happen to know how large the input is~

    let r1 = Regex::new(r"value (\d+) goes to bot (\d+)").unwrap();
    let r2 = Regex::new(r"bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+)").unwrap();

    while let Some(Ok(s)) = input.next() {
        if let Some(caps) = r1.captures(&s) {
            let value = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let bot = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();

            give_bot(&mut bots[bot], value);
            resolve_bot(&mut bots, bot);
        } else if let Some(caps) = r2.captures(&s) {
            let bot = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let type1 = caps.get(2).unwrap().as_str();
            let num1 = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();
            let type2 = caps.get(4).unwrap().as_str();
            let num2 = caps.get(5).unwrap().as_str().parse::<usize>().unwrap();

            bots[bot].lower = match type1 {
                "bot" => Some(Destination::BotNumber(num1)),
                _ => Some(Destination::OutputNumber(num1)),
            };
            bots[bot].upper = match type2 {
                "bot" => Some(Destination::BotNumber(num2)),
                _ => Some(Destination::OutputNumber(num2)),
            };
            resolve_bot(&mut bots, bot);
        }
    }
}

fn give_bot(bot: &mut Bot, value: i32) {
    match bot.contents1 {
        None => bot.contents1 = Some(value),
        Some(_) => bot.contents2 = Some(value),
    }
}

fn output(output: usize, value: i32) {
	/* This solves 10b */
	if output == 0 || output == 1 || output == 2 {
	    println!("Output({}) = {}", output, value);
	}
}

fn resolve_bot(mut bots: &mut [Bot], bot: usize) {
    if !(bots[bot].contents1.is_some() && bots[bot].contents2.is_some() && bots[bot].lower.is_some() && bots[bot].upper.is_some()) {
        return
    }

    let sort = |a, b: i32| { if a > b { (b,a) } else { (a,b) } };
    let (lower, upper) = sort(bots[bot].contents1.unwrap(), bots[bot].contents2.unwrap());

    /* This solves 10a */
    if lower == 17 && upper == 61 {
        println!("Bot {} is comparing 17 and 61", bot);
    }

    match bots[bot].lower.unwrap() {
        Destination::BotNumber(n) => {
            give_bot(&mut bots[n], lower);
            resolve_bot(&mut bots, n);
        },
        Destination::OutputNumber(n) => output(n, lower),
    };
    match bots[bot].upper.unwrap() {
        Destination::BotNumber(n) => {
            give_bot(&mut bots[n], upper);
            resolve_bot(&mut bots, n);
        },
        Destination::OutputNumber(n) => output(n, upper),
    };
}
