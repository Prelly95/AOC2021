use std::error::Error;
use std::fs;
// const INPUT: &str = "./input/test.txt";
const INPUT: &str = "./input/input.txt";

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
	let input = fs::read_to_string(INPUT).expect("Something went wrong reading the file");

	println!("\npart 1:");
	part1(&input)?;
	println!("\npart 2:");
	part2(&input)?;
	Ok(())
}

fn part1(input: &str) -> Result<()> {
	let fuel: Vec<i32> = input.split(",").map(|x| x.parse().unwrap()).collect();
	// Find min pos
	let mut potential_pos: Vec<i32> = Vec::new();
	for from in &fuel {
		let mut move_cost = 0;
		for to in &fuel {
			move_cost += (from - to).abs();
		}
		potential_pos.push(move_cost);
	}
	println!("{:?}", potential_pos.iter().min().unwrap());
	Ok(())
}

fn part2(input: &str) -> Result<()> {
	let fuel: Vec<i32> = input.split(",").map(|x| x.parse().unwrap()).collect();
	// Find min pos
	let mut potential_pos: Vec<i32> = Vec::new();
	let crab_count = fuel.len();
	for to in 0..crab_count {
		let mut move_cost: i32 = 0;
		for from in &fuel {
			let dist = (from - to as i32).abs();
			move_cost += (dist * (dist + 1)) / 2
		}
		potential_pos.push(move_cost)
	}
	println!("{:?}", potential_pos.iter().min().unwrap());
	Ok(())
}
