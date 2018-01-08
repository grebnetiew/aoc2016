use std::io;
use std::io::BufRead;

fn main() {
	let stdin = io::stdin();
	let input = stdin.lock().lines(); // iterator of Result<String>

	let (mut x, mut y) = (0, 2);

	for line in input {
		let (nx, ny) = line.unwrap().chars().map(|c| match c {
			'U' => (0, -1),
			'D' => (0, 1),
			'L' => (-1, 0),
			'R' => (1, 0),
			_ => (0, 0)
		}).fold((x, y), new_value);

		x = nx;
		y = ny;
		print!("{:X}", keypad(x, y));
	}
	println!();
}

fn new_value(current : (i32, i32), delta : (i32, i32)) -> (i32, i32) {
	let candidate = (current.0 + delta.0, current.1 + delta.1);
	if (candidate.0 - 2).abs() + (candidate.1 - 2).abs() > 2 {
		current
	} else {
		candidate
	}
}

fn keypad(x : i32, y : i32) -> i32 {
	match (x, y) {
		(_, 0) => 1,
		(x, 1) => x + 1,
		(x, 2) => x + 5,
		(x, 3) => x + 9,
		(_, 4) => 13,
		(_, _) => 0
	}
}