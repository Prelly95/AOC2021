// use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
	let mut input = String::new();
	io::stdin().read_to_string(&mut input)?;

	println!("\npart 1:");
	part1(&input)?;
	println!("\npart 2:");
	part2(&input)?;
	Ok(())
}

fn part1(input: &str) -> Result<()> {
	let mut prev = 0;
	let mut inc = 0;
	let mut dec = 0;
	for (idx, line) in input.lines().enumerate() {
		if idx < 1 {
			prev = line.parse()?;
			continue;
		}
		let current: i32 = line.parse()?;
		if current > prev {
			inc = inc + 1;
			println!("increment: c {}, p {}", current, prev)
		} else if current < prev {
			dec = dec + 1;
			println!("decrement: c {}, p{}", current, prev)
		} else {
			println!("no change")
		}
		prev = current;
	}
	println!("inc: {}", inc);
	println!("dec: {}", dec);
	Ok(())
}

fn part2(input: &str) -> Result<()> {
	let mut arr: Vec<i32> = vec![];
	let mut inc = 0;
	let mut dec = 0;
	for line in input.lines() {
		arr.push(line.parse()?);
	}
	for ii in 0..arr.len() - 3 {
		let a = arr[ii] + arr[ii + 1] + arr[ii + 2];
		let b = arr[ii + 1] + arr[ii + 2] + arr[ii + 3];
		if a < b {
			inc = inc + 1;
			println!("increment: c {}, p {}", a, b)
		} else if a > b {
			dec = dec + 1;
			println!("decrement: c {}, p{}", a, b)
		} else {
			println!("no change")
		}
	}
	println!("inc: {}", inc);
	println!("dec: {}", dec);
	Ok(())
}
