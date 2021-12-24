use std::collections::HashMap;
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
	let mut grid: HashMap<(i32, i32), u8> = HashMap::new();
	let mut x_dim = 0;
	let mut y_dim = 0;
	let mut flash_count = 0;
	for (y, line) in input.lines().enumerate() {
		for (x, d) in line.chars().map(|x| x.to_digit(10u32).unwrap()).enumerate() {
			grid.insert((x as i32, y as i32), d as u8);
			x_dim = x + 1;
		}
		y_dim = y + 1;
	}

	for _ in 0..100 {
		// Step 1
		for y in 0..y_dim {
			for x in 0..x_dim {
				if let Some(o) = grid.get_mut(&(x as i32, y as i32)) {
					*o += 1;
				}
			}
		}
		// Step 2
		let mut add_one: Vec<(i32, i32)> = Vec::new();
		let mut flashed: Vec<(i32, i32)> = Vec::new();
		let mut flashing = true;
		while flashing {
			for y in 0..y_dim {
				for x in 0..x_dim {
					if let Some(o) = grid.get_mut(&(x as i32, y as i32)) {
						if !flashed.contains(&(x as i32, y as i32)) {
							if *o > 9 {
								flashed.push((x as i32, y as i32));
								add_one.push((x as i32, y as i32));
								for (dx, dy) in &[
									(0, 1),
									(0, -1),
									(1, 0),
									(-1, 0),
									(1, 1),
									(1, -1),
									(-1, 1),
									(-1, -1),
								] {
									add_one.push((x as i32 + dx, y as i32 + dy));
								}
							}
						}
					}
				}
			}
			flashing = add_one.len() > 0;
			while let Some((x_p1, y_p1)) = add_one.pop() {
				if let Some(o) = grid.get_mut(&(x_p1 as i32, y_p1 as i32)) {
					*o += 1;
				}
			}
		}
		while let Some((x_p1, y_p1)) = flashed.pop() {
			if let Some(o) = grid.get_mut(&(x_p1 as i32, y_p1 as i32)) {
				*o = 0;
				flash_count += 1
			}
		}
		// print_grid(&grid, x_dim, y_dim);
	}
	println!("{}", flash_count);
	Ok(())
}

fn part2(input: &str) -> Result<()> {
	let mut grid: HashMap<(i32, i32), u8> = HashMap::new();
	let mut x_dim = 0;
	let mut y_dim = 0;
	for (y, line) in input.lines().enumerate() {
		for (x, d) in line.chars().map(|x| x.to_digit(10u32).unwrap()).enumerate() {
			grid.insert((x as i32, y as i32), d as u8);
			x_dim = x + 1;
		}
		y_dim = y + 1;
	}
	let mut synced = false;
	let mut sync_idx = 0;
	while !synced {
		// Step 1
		for y in 0..y_dim {
			for x in 0..x_dim {
				if let Some(o) = grid.get_mut(&(x as i32, y as i32)) {
					*o += 1;
				}
			}
		}
		// Step 2
		let mut add_one: Vec<(i32, i32)> = Vec::new();
		let mut flashed: Vec<(i32, i32)> = Vec::new();
		let mut flashing = true;
		while flashing {
			for y in 0..y_dim {
				for x in 0..x_dim {
					if let Some(o) = grid.get_mut(&(x as i32, y as i32)) {
						if !flashed.contains(&(x as i32, y as i32)) {
							if *o > 9 {
								flashed.push((x as i32, y as i32));
								add_one.push((x as i32, y as i32));
								for (dx, dy) in &[
									(0, 1),
									(0, -1),
									(1, 0),
									(-1, 0),
									(1, 1),
									(1, -1),
									(-1, 1),
									(-1, -1),
								] {
									add_one.push((x as i32 + dx, y as i32 + dy));
								}
							}
						}
					}
				}
			}
			flashing = add_one.len() > 0;
			while let Some((x_p1, y_p1)) = add_one.pop() {
				if let Some(o) = grid.get_mut(&(x_p1 as i32, y_p1 as i32)) {
					*o += 1;
				}
			}
		}
		while let Some((x_p1, y_p1)) = flashed.pop() {
			if let Some(o) = grid.get_mut(&(x_p1 as i32, y_p1 as i32)) {
				*o = 0;
			}
		}
		synced = grid.iter().fold(true, |a, x| a && *x.1 == 0);
		sync_idx += 1;
		// print_grid(&grid, x_dim, y_dim);
		// println!("{}", sync_idx);
	}
	println!("{}", sync_idx);
	Ok(())
}

fn print_grid(grid: &HashMap<(i32, i32), u8>, x_dim: usize, y_dim: usize) {
	for y in 0..y_dim {
		for x in 0..x_dim {
			print!("{:0>2?} ", grid.get(&(x as i32, y as i32)).unwrap());
		}
		println!("");
	}
	println!("");
}
