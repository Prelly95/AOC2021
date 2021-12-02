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
	let mut h = 0;
	let mut v = 0;
	for line in input.lines() {
		
		let tmp: Vec<&str> = line.split(" ").collect();
		let u: i32 = tmp[1].parse()?;
		match tmp[0] {
			"forward" => {
				h = h + u;
			}
			"up" => {
				v = v - u;
			}
			"down" => {
				v = v + u;
			}
			_ => (),
		}
	}
	println!("{}", h*v);
	Ok(())
}

fn part2(input: &str) -> Result<()> {
	let mut h = 0;
	let mut v = 0;
	let mut a = 0;
	for line in input.lines() {
		
		let tmp: Vec<&str> = line.split(" ").collect();
		let u: i32 = tmp[1].parse()?;
		match tmp[0] {
			"forward" => {
				h = h + u;
				v = v + a*u;
			}
			"up" => {
				a = a - u;
			}
			"down" => {
				a = a + u;
			}
			_ => (),
		}
	}
	println!("{}", h*v);
	Ok(())
}
