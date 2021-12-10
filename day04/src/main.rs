use std::error::Error;
use std::fs;

#[derive(Debug, Clone)]
struct Board {
	winner: bool,
	rows: Vec<Vec<(u32, bool)>>,
	cols: Vec<Vec<(u32, bool)>>,
}

impl Board {
	fn from_str(str_in: &str) -> Board {
		let mut rows = vec![vec![(0u32, false); 5]; 5];
		let mut cols = vec![vec![(0u32, false); 5]; 5];
		for (idx, line) in str_in.lines().enumerate() {
			rows[idx] = line
				.trim()
				.split(" ")
				.filter(|&x| x != "")
				.map(|x| (x.parse().unwrap(), false))
				.collect::<Vec<(u32, bool)>>();
		}
		for (ii, r) in rows.iter().enumerate() {
			for (jj, &e) in r.iter().enumerate() {
				cols[jj][ii] = e;
			}
		}
		return Board {
			winner: false,
			rows: rows,
			cols: cols,
		};
	}

	fn mark_number(&mut self, num: u32) -> Option<u32> {
		for ii in 0..self.rows.len() {
			for jj in 0..self.rows[0].len() {
				if self.rows[ii][jj].0 == num {
					self.rows[ii][jj].1 = true;
					match self.validate(num) {
						Some(w) => {
							return Some(w);
						}
						None => (),
					}
				}
			}
		}
		for ii in 0..self.cols.len() {
			for jj in 0..self.cols[0].len() {
				if self.cols[ii][jj].0 == num {
					self.cols[ii][jj].1 = true;
					match self.validate(num) {
						Some(w) => {
							return Some(w);
						}
						None => (),
					}
				}
			}
		}
		None
	}

	fn score(&self, winning_num: u32) -> u32 {
		let mut f_sum: u32 = 0;
		for r in self.rows.iter() {
			let r_sum = r
				.iter()
				.filter(|(_, x)| *x != true)
				.fold(0, |sum, (n, _)| sum + n);
			f_sum = f_sum + r_sum
		}
		return f_sum * winning_num;
	}

	fn validate(&self, winning_num: u32) -> Option<u32> {
		for r in self.rows.iter() {
			if r.iter().all(|&(_, x)| x == true) {
				return Some(self.score(winning_num));
			}
		}
		for c in self.cols.iter() {
			if c.iter().all(|&(_, x)| x == true) {
				return Some(self.score(winning_num));
			}
		}
		None
	}
}

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
	let mut split_input = input.split("\n\n");
	let winning_numbers = split_input
		.next()
		.unwrap()
		.trim()
		.split(",")
		.map(|x| x.parse().unwrap())
		.collect::<Vec<u32>>();

	let mut boards: Vec<Board> = Vec::new();
	for b in split_input {
		boards.push(Board::from_str(b).clone());
	}
	for n in winning_numbers {
		for b in &mut boards {
			match b.mark_number(n) {
				Some(w) => {
					println!("{}", w);
					return Ok(());
				}
				None => (),
			}
		}
	}
	Ok(())
}

fn part2(input: &str) -> Result<()> {
	let mut split_input = input.split("\n\n");
	let winning_numbers = split_input
		.next()
		.unwrap()
		.trim()
		.split(",")
		.map(|x| x.parse().unwrap())
		.collect::<Vec<u32>>();

	let mut boards: Vec<Board> = Vec::new();
	for b in split_input {
		boards.push(Board::from_str(b).clone());
	}
	let mut winning_scores = Vec::new();
	for n in winning_numbers {
		for b in &mut boards {
			if !b.winner {
				match b.mark_number(n) {
					Some(w) => {
						winning_scores.push(w);
						b.winner = true;
					}
					None => (),
				}
			}
		}
	}
	println!("{}", winning_scores[winning_scores.len()-1]);
	Ok(())
}
