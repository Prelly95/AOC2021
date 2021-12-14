use std::collections::HashMap;
use std::error::Error;
use std::fs;

// const INPUT: &str = "./input/test.txt";
const INPUT: &str = "./input/input.txt";

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
struct Pattern {
	alpha: u8,
	beta: u8,
	gamma: u8,
	delta: [u8; 3],
	eps: [u8; 3],
	zeta: u8,
	code: HashMap<char, u8>,
}
impl Pattern {
	fn from_str(row: &str) -> Pattern {
		let mut a = 0;
		let mut b = 0;
		let mut g = 0;
		let mut d = [0; 3];
		let mut e = [0; 3];
		let mut z = 0;
		// parse pattern into struct
		for digit in row.split(" ") {
			let active_segments = digit.chars().count();
			match active_segments {
				2 => {
					for c in digit.chars() {
						let bit = c as u8 - 97;
						a = a | 1 << bit;
					}
				}
				3 => {
					for c in digit.chars() {
						let bit = c as u8 - 97;
						b = b | 1 << bit;
					}
				}
				4 => {
					for c in digit.chars() {
						let bit = c as u8 - 97;
						g = g | 1 << bit;
					}
				}
				5 => {
					for ii in &mut d {
						if ii == &0 {
							for c in digit.chars() {
								let bit = c as u8 - 97;
								*ii = *ii | 1 << bit;
							}
							break;
						}
					}
				}
				6 => {
					for ii in &mut e {
						if *ii == 0 {
							for c in digit.chars() {
								let bit = c as u8 - 97;
								*ii = *ii | 1 << bit;
							}
							break;
						}
					}
				}
				7 => {
					for c in digit.chars() {
						let bit = c as u8 - 97;
						z = z | 1 << bit;
					}
				}
				_ => (),
			}
		}
		return Pattern {
			alpha: a,
			beta: b,
			gamma: g,
			delta: d,
			eps: e,
			zeta: z,
			code: HashMap::new(),
		};
	}
	fn byte_to_char(&self, byte: u8) -> Option<char> {
		if byte.count_ones() != 1 {
			return None;
		}
		let mut idx: u8 = 0;
		while (byte & !(1 << idx)) > 0 {
			idx += 1;
		}
		return Some((idx + 97) as char);
	}

	fn crack_code(&mut self) {
		let seg_0_bit = self.alpha ^ self.beta;
		self.code.insert(self.byte_to_char(seg_0_bit).unwrap(), 1);

		let seg_1_bit = (!(self.eps[0] & self.eps[1] & self.eps[2]) & 0b01111111) & self.alpha;
		self.code.insert(self.byte_to_char(seg_1_bit).unwrap(), 2);
		let seg_2_bit = (self.eps[0] & self.eps[1] & self.eps[2]) & self.alpha;
		self.code.insert(self.byte_to_char(seg_2_bit).unwrap(), 4);

		let seg_3_bit =
			!((!(self.eps[0] & self.eps[1] & self.eps[2]) & 0b01111111) | (self.beta) | self.gamma)
				& 0b01111111;
		self.code.insert(self.byte_to_char(seg_3_bit).unwrap(), 8);
		let seg_4_bit = !((self.eps[0] & self.eps[1] & self.eps[2]) & 0b01111111)
			& ((!self.gamma) & 0b01111111);
		self.code.insert(self.byte_to_char(seg_4_bit).unwrap(), 16);

		let seg_5_bit = !((!(self.eps[0] & self.eps[1] & self.eps[2]) & 0b01111111)
			| self.alpha | (self.delta[0] & self.delta[1] & self.delta[2]))
			& 0b01111111;	
		self.code.insert(self.byte_to_char(seg_5_bit).unwrap(), 32);

		let seg_6_bit = (self.delta[0] & self.delta[1] & self.delta[2]) & (self.gamma);
		self.code.insert(self.byte_to_char(seg_6_bit).unwrap(), 64);
	}

	fn decode_digit(&self, scrambled_digit: &str) -> u8 {
		let mut active_segments = 0;
		for c in scrambled_digit.chars() {
			active_segments |= self.code[&c];
		}
		return match active_segments {
			0b01101111 => 9,
			0b01111111 => 8,
			0b00000111 => 7,
			0b01111101 => 6,
			0b01101101 => 5,
			0b01100110 => 4,
			0b01001111 => 3,
			0b01011011 => 2,
			0b00000110 => 1,
			0b00111111 => 0,
			_ => !0,
		};
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
	let mut unique_count = 0;
	for line in input.lines() {
		let row: Vec<&str> = line.split(" | ").collect();
		unique_count += row[1].split(" ").fold(0, |acc, x| {
			let c = x.len();
			if c == 2 || c == 4 || c == 3 || c == 7 {
				return acc + 1;
			} else {
				return acc;
			}
		})
	}
	println!("{}", unique_count);
	Ok(())
}

fn part2(input: &str) -> Result<()> {
	let mut sum = 0;
	for line in input.lines() {
		let row: Vec<&str> = line.split(" | ").collect();
		let mut pat = Pattern::from_str(row[0]);
		pat.crack_code();
		let mut num = 0;
		for (ii, x) in row[1].split(" ").enumerate() {
			let digit = pat.decode_digit(x);
			num += digit as i32 * 10i32.pow(3-ii as u32);
		}
		sum += num;
	}
	println!("{}", sum);
	Ok(())
}
