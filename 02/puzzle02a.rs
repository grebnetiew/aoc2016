use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines(); // iterator of Result<String>

    let (mut x, mut y) = (1, 1);

    for line in input {
        let (nx, ny) = line.unwrap()
            .chars()
            .map(|c| match c {
                'U' => (0, -1),
                'D' => (0, 1),
                'L' => (-1, 0),
                'R' => (1, 0),
                _ => (0, 0),
            })
            .fold((x, y), |sum, val| {
                (limit(sum.0 + val.0, 0, 2), limit(sum.1 + val.1, 0, 2))
            });

        x = nx;
        y = ny;
        print!("{}", 3 * y + x + 1);
    }
    println!();
}

fn limit(x: i32, lower: i32, upper: i32) -> i32 {
    if x < lower {
        lower
    } else if x > upper {
        upper
    } else {
        x
    }
}
