use std::error::Error;
use std::fs;

const INPUT: &str = "./input/test.txt";
// const INPUT: &str = "./input/input.txt";

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
	let input = fs::read_to_string(INPUT).expect("Something went wrong reading the file");

	println!("\npart 1:");
	part1(&input)?;
	println!("\npart 2:");
	// part2(&input)?;
	Ok(())
}

fn part1(input: &str) -> Result<()> {
	let map:Vec<Vec<u8>> = input.lines().map(|x| x.chars().map(|y| y.to_digit(10u32).unwrap() as u8).collect()).collect();
	println!("map:");
	for m in map{
		println!("{:?}", m);
	}
	Ok(())
}

fn part2(input: &str) -> Result<()> {
	todo!();
	Ok(())
}
