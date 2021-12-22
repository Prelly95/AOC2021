use std::collections::HashMap;
use std::error::Error;
use std::fs;

// const INPUT: &str = "./input/test.txt";
const INPUT: &str = "./input/input.txt";

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
	let input = fs::read_to_string(INPUT).expect("Something went wrong reading the file");
	let mut height_map = HashMap::new();
	let mut x_dim = 0;
	let mut y_dim = 0;
	// read map into array
	for (y, line) in input.lines().enumerate() {
		for (x, c) in line.chars().enumerate() {
			x_dim = x as i32;
			y_dim = y as i32;
			height_map.insert((x as i32, y as i32), c.to_digit(10u32).unwrap() as u8);
		}
	}
	println!("\npart 1:");
	part1(&height_map, (x_dim, y_dim))?;
	println!("\npart 2:");
	part2(&height_map, (x_dim, y_dim))?;
	Ok(())
}

fn part1(height_map: &HashMap<(i32, i32), u8>, dim: (i32, i32)) -> Result<()> {
	// read map into padded array
	let mut low_points: Vec<u8> = Vec::new();
	for y in 0..=(dim.1) {
		for x in 0..=(dim.0) {
			let poi = height_map.get(&(x, y)).unwrap();
			let mut is_low = true;
			for (dx, dy) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
				if let Some(v) = height_map.get(&(x + dx, y + dy)) {
					if v <= poi {
						is_low = false;
						break;
					}
				}
			}
			if is_low {
				low_points.push(*poi);
			}
		}
	}
	let score = low_points.iter().fold(0, |a, h| a + (h + 1) as u32);
	println!("{:?}", score);
	Ok(())
}

fn part2(height_map: &HashMap<(i32, i32), u8>, dim: (i32, i32)) -> Result<()> {
	// read map into padded array
	let mut low_points = HashMap::new();
	for y in 0..=(dim.1) {
		for x in 0..=(dim.0) {
			let poi = height_map.get(&(x, y)).unwrap();
			let mut is_low = true;
			for (dx, dy) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
				if let Some(v) = height_map.get(&(x + dx, y + dy)) {
					if v <= poi {
						is_low = false;
						break;
					}
				}
			}
			if is_low {
				low_points.insert((x, y), *poi);
			}
		}
	}
	let mut scores: Vec<u32> = Vec::new();
	let mut searched = HashMap::new();
	for l_p in low_points {
		// let mut b_s = 0;
		let mut basin = vec![l_p];
		let mut basin_score = 1;
		// println!("{:?}", l_p.0);
		// print!("{}", l_p.1);
		while let Some(poi) = basin.pop() {
			for (dx, dy) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
				let search_point = (poi.0 .0 + dx, poi.0 .1 + dy);
				if let Some(v) = height_map.get(&search_point) {
					if *v > poi.1 && *v != 9 && !searched.contains_key(&search_point) {
						{
							searched.insert(search_point, true);
							basin.push(((poi.0 .0 + dx, poi.0 .1 + dy), *v));
							basin_score += 1;
							// print!("{}", v);
						}
					}
				}
			}
		}
		scores.push(basin_score);
	}
	scores.sort();
	println!("{:?}", scores[scores.len()-3] * scores[scores.len()-2] * scores[scores.len()-1]);
	Ok(())
}
