use std::io; // provides io's stdin()
use std::io::BufRead; // provides lines()
use std::collections::HashSet;

fn main() {
	// Get the first line from standard input
	let stdin = io::stdin(); // need to lock the stdin for synced access like "next line"
		// This needs to be a separate variable so it lives until the .fold at the end

	let input : String = stdin.lock() // The type is optional, but I want to be sure
		.lines() // no newline bytes; is an iterator of Result<String> (which is String or Err)
		.next() // only the first line is relevant; is an Option<_> (which is Result<_> or None)
		.unwrap() // returns the Result<_> in Option<_>, or panics if there is None
		.unwrap(); // returns the String in Result<String>, or panics if it's an error

	// Part A. Find out how far you are from the starting point

	let directions : Vec<(i32, i32)> = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];

	let (_, x, y) : (usize, i32, i32) = // a tuple of integers
		input.split(", ") // split it at the given pattern; is an iterator of Strings
		.map(|s| s.split_at(1)) // gives us (&str, &str) like ("L", "12")
		.map(|(s, n)| (match s {"L" => 3, "R" => 1,	_ => panic!(format!("{:?}", s))},
		               n.parse::<i32>().unwrap())) // it becomes (3, 12)
		.fold((0, 0, 0), |(d, x, y), (dir, dist)| { 
			let nd = (d + dir) % 4; 
			(nd, x + dist * directions[nd].0, y + dist * directions[nd].1)
		}); // and adds them up
			// This last line is a consumer, and it is the only reason the otherwise lazy
			// iterators will actually do anything

	println!("{}", x.abs() + y.abs());

	// Part B. Stop at the first revisited node.

	let instructions : Vec<(usize, i32)> = input.split(", ").map(|s| s.split_at(1))
		.map(|(s, n)| (match s {"L" => 3, "R" => 1,	_ => panic!(format!("{:?}", s))},
		               n.parse::<i32>().unwrap())) // it becomes (3, 12)
		.collect(); // from iterator to vector

	let mut visited = HashSet::new();
	let (mut dir, mut x, mut y) = (0usize, 0i32, 0i32);
	visited.insert((x,y));

'search:
	for instruction in instructions {
		dir = (dir + instruction.0) % 4;
		for _ in 0..instruction.1 {
			x += directions[dir].0;
			y += directions[dir].1;
			if !visited.insert((x,y)) {
				println!("{}", x.abs() + y.abs());
				break 'search
			}
		}
	}
}
