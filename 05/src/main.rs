extern crate crypto;
use crypto::md5::Md5;
use crypto::digest::Digest;

use std::io;
use std::io::Write;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let input: String = stdin.lock().lines().next().unwrap().unwrap();

    // Part A

    let mut charsfound = 0;
    let mut hasher = Md5::new();
    let mut hash: [u8; 16] = [0; 16];
    let mut i = 0u32;

    while charsfound != 8 {
        let candidate = input.clone() + &i.to_string();
        hasher.input_str(&candidate);

        hasher.result(&mut hash);
        if hash[0] == 0 && hash[1] == 0 && hash[2] >> 4 == 0 {
            print!("{:x}", hash[2] & 0x0f);
            charsfound += 1;
        }

        hasher.reset();
        i += 1;
    }
    println!();

    // Part B

    let mut charsfound = 0;
    let mut i = 0u32;
    let mut password: [char; 8] = ['_'; 8];

    while charsfound != 8 {
        let candidate = input.clone() + &i.to_string();
        hasher.input_str(&candidate);

        hasher.result(&mut hash);
        if hash[0] == 0 && hash[1] == 0 && hash[2] >> 4 == 0 && hash[2] & 0x0f < 8 &&
            password[(hash[2] & 0x0f) as usize] == '_'
        {
            password[(hash[2] & 0x0f) as usize] =
                format!("{:x}", hash[3] >> 4).chars().next().unwrap();
            charsfound += 1;

            print!("\r");
            for c in password.iter() {
                print!("{}", c);
            }
            let _ = io::stdout().flush();
        }

        hasher.reset();
        i += 1;
    }
    println!();
}
