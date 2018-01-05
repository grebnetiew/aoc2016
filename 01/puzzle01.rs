use std::io; // provides io's stdin()
use std::io::BufRead; // provides lines()

fn main() {
	// Get the first line from standard input
	let stdin = io::stdin(); // need to lock the stdin for synced access like "next line"
		// This needs to be a separate variable so it lives until the .fold at the end

	let input : String = stdin.lock() // The type is optional, but I want to be sure
		.lines() // no newline bytes; is an iterator of Result<String> (which is String or Err)
		.next() // only the first line is relevant; is an Option<_> (which is Result<_> or None)
		.unwrap() // returns the Result<_> in Option<_>, or panics if there is None
		.unwrap(); // returns the string in Result<String>, or panics if it's an error

	let (x, y) : (i32, i32) = // a tuple of integers
		input.split(", ") // split it at the given pattern
		.map(|s| s.split_at(1)) // gives us (&str, &str) like ("L", "12")
		.map(|(s, n)| (s, n.parse::<i32>().unwrap())) // it becomes ("L", 12)
		.map(|pair| match pair {
			("L", n) => (-n, 0),
			("R", n) => (n, 0),
			("U", n) => (0, -n),
			("D", n) => (0, n),
			_ => panic!(pair)
		}) // turns them into relative position coordinates like (-12, 0)
		.fold((0, 0), |(x, y), (dx, dy)| (x + dx, y + dy)); // and adds them up
			// This last line is a consumer, and it is the only reason the otherwise lazy
			// iterators will actually do anything

	println!("{}", x.abs() + y.abs())
}