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
	let mut r_arr: Vec<Vec<u32>> = Vec::new();
	for line in input.lines() {
		let l_arr = line.chars();
		let mut t_arr: Vec<u32> = vec![];
		for b in l_arr {
			t_arr.push(b.to_digit(2).unwrap());
		}
		// println!("{:?}", t_arr);
		r_arr.push(t_arr)
	}
	let short = r_arr[0].len();
	let long = r_arr.len();

	let mut c_arr: Vec<Vec<u32>> = vec![vec![1u32; long]; short];
	for ii in 0..long {
		for jj in 0..short {
			c_arr[jj][ii] = r_arr[ii][jj];
		}
	}
	let mut eps: u32 = 0;
	let mut gam: u32 = 0;
	for (idx, c) in c_arr.into_iter().enumerate() {
		let zeros: Vec<u32> = c.into_iter().filter(|&x| x == 0u32).collect::<Vec<u32>>();
		let z = if zeros.len() > (long - zeros.len()) {
			0
		} else {
			1
		};
		let o = if zeros.len() > (long - zeros.len()) {
			1
		} else {
			0
		};
		eps = eps | z << short - idx - 1;
		gam = gam | o << short - idx - 1;
	}
	println!("{}", eps * gam);

	Ok(())
}

fn part2(input: &str) -> Result<()> {
	let mut r_arr: Vec<Vec<u32>> = Vec::new();
	for line in input.lines() {
		let l_arr = line.chars();
		let mut t_arr: Vec<u32> = vec![];
		for b in l_arr {
			t_arr.push(b.to_digit(2).unwrap());
		}
		// println!("{:?}", t_arr);
		r_arr.push(t_arr)
	}
	let ox = oxy(&mut r_arr.clone(), &mut 0usize);
	let c2 = c02(&mut r_arr.clone(), &mut 0usize);

	let a = ox.iter().fold(0, |result, &bit| (result << 1) ^ bit);
	let b = c2.iter().fold(0, |result, &bit| (result << 1) ^ bit);

	println!("{:?}", a*b);
	Ok(())
}

fn oxy(row_vec: &mut Vec<Vec<u32>>, bit: &mut usize) -> Vec<u32> {
	let col = row_vec.len();
	let row = row_vec[0].len();
	let mut c_arr: Vec<Vec<u32>> = vec![vec![1u32; col]; row];
	for ii in 0..col {
		for jj in 0..row {
			c_arr[jj][ii] = row_vec[ii][jj].clone();
		}
	}
	let zeros: &Vec<u32> = &c_arr[*bit]
		.clone()
		.into_iter()
		.filter(|&x| x == 0u32)
		.collect::<Vec<u32>>();
	let common = if zeros.len() as i32 > (col as i32 - zeros.len() as i32) {
		0
	} else {
		1
	};
	let mut filtered_vec: Vec<Vec<u32>> = row_vec
		.clone()
		.into_iter()
		.filter(|x| x[*bit] == common)
		.collect();

	if !(filtered_vec.len() > 1) {
		return filtered_vec[0].clone();
	}
	*bit = *bit + 1;
	if *bit > row - 1 {
		return filtered_vec[0].clone();
	}
	oxy(&mut filtered_vec, bit)
}

fn c02(row_vec: &mut Vec<Vec<u32>>, bit: &mut usize) -> Vec<u32> {
	let col = row_vec.len();
	let row = row_vec[0].len();
	let mut c_arr: Vec<Vec<u32>> = vec![vec![1u32; col]; row];
	for ii in 0..col {
		for jj in 0..row {
			c_arr[jj][ii] = row_vec[ii][jj].clone();
		}
	}
	let zeros: &Vec<u32> = &c_arr[*bit]
		.clone()
		.into_iter()
		.filter(|&x| x == 0u32)
		.collect::<Vec<u32>>();
	let common = if (zeros.len() as i32) <= (col as i32 - zeros.len() as i32) {
		0
	} else {
		1
	};
	let mut filtered_vec: Vec<Vec<u32>> = row_vec
		.clone()
		.into_iter()
		.filter(|x| x[*bit] == common)
		.collect();

	*bit = *bit + 1;
	if !(filtered_vec.len() > 1) {
		return filtered_vec[0].clone();
	}
	if *bit > filtered_vec[0].len() - 2 {
		return filtered_vec[0].clone();
	}
	c02(&mut filtered_vec, bit)
}
