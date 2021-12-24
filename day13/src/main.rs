use std::error::Error;
use std::fs;
use std::collections::HashSet;

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
	let (coords, fold) = input.split_once("\n\n").unwrap();
	let mut dots: Vec<(usize, usize)> = coords
		.lines()
		.map(|s| {
			let (x, y) = &s.split_once(",").unwrap();
			return (x.parse().unwrap(), y.parse().unwrap());
		})
		.collect();

	let (axis_str, loc_str) = fold.lines().next().unwrap().split_once("=").unwrap();
	let axis = &axis_str[axis_str.len() - 1..];
	let loc: usize = loc_str.parse().unwrap();
	match axis {
		"x" => {
			for d in &mut dots {
				if d.0 > loc {
					d.0 = loc - (d.0 - loc);
				}
			}
		}
		"y" => {
			for d in &mut dots {
				if d.1 > loc {
					d.1 = loc - (d.1 - loc);
				}
			}
		}
		_ => (),
	}
	println!("{}", dots.iter().collect::<HashSet<_>>().len());
	Ok(())
}

fn part2(input: &str) -> Result<()> {
	let (coords, fold) = input.split_once("\n\n").unwrap();
	let mut dots: Vec<(usize, usize)> = coords
		.lines()
		.map(|s| {
			let (x, y) = &s.split_once(",").unwrap();
			return (x.parse().unwrap(), y.parse().unwrap());
		})
		.collect();

	for ii in fold.lines() {
		let (axis_str, loc_str) = ii.split_once("=").unwrap();
		let axis = &axis_str[axis_str.len() - 1..];
		let loc: usize = loc_str.parse().unwrap();
		match axis {
			"x" => {
				for d in &mut dots {
					if d.0 > loc {
						d.0 = loc - (d.0 - loc);
					}
				}
			}
			"y" => {
				for d in &mut dots {
					if d.1 > loc {
						d.1 = loc - (d.1 - loc);
					}
				}
			}
			_ => (),
		}
	}
	let x_dim = dots.iter().map(|(x, _)| *x as usize).max().unwrap() + 1;
	let y_dim = dots.iter().map(|(_, y)| *y as usize).max().unwrap() + 1;
	let mut display: Vec<Vec<char>> = vec![vec![' '; x_dim]; y_dim];
	for (x, y) in dots {
		display[y][x] = '0';
	}
	for l in display {
		for p in l {
			print!("{}", p);
		}
		println!("");
	}

	Ok(())
}
