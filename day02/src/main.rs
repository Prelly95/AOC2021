// use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
	let mut input = String::new();
	io::stdin().read_to_string(&mut input)?;

	println!("\npart 1:\n_______________________________________________");
	part1(&input)?;
	println!("\npart 2:\n_______________________________________________");
	part2(&input)?;
	Ok(())
}

fn part1(input: &str) -> Result<()> {

	for line in input.lines() {
		let n: i32 = line.parse()?;
		println!("{}", n)
	}
	Ok(())
}

fn part2(input: &str) -> Result<()> {

	for line in input.lines() {
		let n: i32 = line.parse()?;
		println!("{}", n)
	}
	Ok(())
}
