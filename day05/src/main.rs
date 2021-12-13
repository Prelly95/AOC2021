use std::collections::hash_map;
use std::error::Error;
use std::fs;
// const INPUT: &str = "./input/test.txt";
const INPUT: &str = "./input/input.txt";

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
struct Coordinate {
	x: i32,
	y: i32,
}
impl Coordinate {
	fn new(coord_string: &str) -> Coordinate {
		let xy: Vec<i32> = coord_string
			.split(",")
			.map(|x| x.parse().unwrap())
			.collect();
		return Coordinate { x: xy[0], y: xy[1] };
	}
}

fn main() -> Result<()> {
	let input = fs::read_to_string(INPUT).expect("Something went wrong reading the file");

	println!("\npart 1:");
	part1(&input)?;
	println!("\npart 2:");
	part2(&input)?;
	Ok(())
}

fn part1(input: &str) -> Result<()> {
	let mut grid = vec![vec![0u8; 1000]; 1000];

	for inp_line in input.lines() {
		let coords: Vec<&str> = inp_line.split(" -> ").collect();
		let p1 = Coordinate::new(coords[0]);
		let p2 = Coordinate::new(coords[1]);
		let dx = p2.x - p1.x;
		let dy = p2.y - p1.y;
		if dx != 0 {
			if dy == 0 {
				let sn = dx.signum();
				for ii in 0..dx.abs() + 1 {
					grid[(p1.x + sn * ii) as usize][p1.y as usize] += 1;
				}
			}
		} else if dy != 0 {
			if dx == 0 {
				let sn = dy.signum();
				for ii in 0..dy.abs() + 1 {
					grid[p1.x as usize][(p1.y + sn * ii) as usize] += 1;
				}
			}
		}
	}
	let mut count = 0;
	for row in grid {
		for col in row {
			if col > 1u8 {
				count += 1;
			}
		}
	}
	println!("{:?}", count);
	Ok(())
}

fn part2(input: &str) -> Result<()> {
	let mut grid = vec![vec![0u8; 1000]; 1000];

	for inp_line in input.lines() {
		let coords: Vec<&str> = inp_line.split(" -> ").collect();
		let p1 = Coordinate::new(coords[0]);
		let p2 = Coordinate::new(coords[1]);
		let dx = p2.x - p1.x;
		let dy = p2.y - p1.y;

		let sx = dx.signum();
		let sy = dy.signum();
		if dx != 0 && dy != 0 {
			for ii in 0..dx.abs() + 1 {
				for jj in 0..dy.abs() + 1 {
					if jj == ii {
						grid[(p1.x + sx * ii) as usize][(p1.y + sy * jj) as usize] += 1;
					}
				}
			}
		} else {
			if dx != 0 {
				let sn = dx.signum();
				for ii in 0..dx.abs() + 1 {
					grid[(p1.x + sn * ii) as usize][p1.y as usize] += 1;
				}
			} else if dy != 0 {
				let sn = dy.signum();
				for ii in 0..dy.abs() + 1 {
					grid[p1.x as usize][(p1.y + sn * ii) as usize] += 1;
				}
			}
		}
	}
	let mut count = 0;
	for row in grid {
		for col in row {
			if col > 1u8 {
				count += 1;
			}
		}
	}
	println!("{:?}", count);
	Ok(())
}
