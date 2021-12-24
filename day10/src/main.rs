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
	let mut score = 0;
	for line in input.lines() {
		let mut open: Vec<char> = Vec::new();
		let mut line_score = 0;
		for c in line.chars() {
			match c {
				// Opening brackets
				'(' => open.push('('),
				'[' => open.push('['),
				'{' => open.push('{'),
				'<' => open.push('<'),
				// closing brackets
				')' => {
					if open[open.len() - 1] == '(' {
						open.pop();
					} else {
						line_score += 3;
						break;
					}
				}
				']' => {
					if open[open.len() - 1] == '[' {
						open.pop();
					} else {
						line_score += 57;
						break;
					}
				}
				'}' => {
					if open[open.len() - 1] == '{' {
						open.pop();
					} else {
						line_score += 1197;
						break;
					}
				}
				'>' => {
					if open[open.len() - 1] == '<' {
						open.pop();
					} else {
						line_score += 25137;
						break;
					}
				}
				_ => (),
			}
		}
		score += line_score;
	}
	println!("score: {:?}", score);
	Ok(())
}

fn part2(input: &str) -> Result<()> {
	let mut score: Vec<u64> = Vec::new();
	for line in input.lines() {
		let mut open: Vec<char> = Vec::new();
		let mut corrupt = false;
		for c in line.chars() {
			match c {
				// Opening brackets
				'(' => open.push('('),
				'[' => open.push('['),
				'{' => open.push('{'),
				'<' => open.push('<'),
				// closing brackets
				')' => {
					if open[open.len() - 1] == '(' {
						open.pop();
					} else {
						corrupt = true;
						break;
					}
				}
				']' => {
					if open[open.len() - 1] == '[' {
						open.pop();
					} else {
						corrupt = true;
						break;
					}
				}
				'}' => {
					if open[open.len() - 1] == '{' {
						open.pop();
					} else {
						corrupt = true;
						break;
					}
				}
				'>' => {
					if open[open.len() - 1] == '<' {
						open.pop();
					} else {
						corrupt = true;
						break;
					}
				}
				_ => (),
			}
		}
		if !corrupt
		{
			let mut line_score = 0;
			for o in open.iter().rev(){
				match o{
					'(' => {line_score *= 5;line_score += 1}
					'[' => {line_score *= 5;line_score += 2}
					'{' => {line_score *= 5;line_score += 3}
					'<' => {line_score *= 5;line_score += 4}
					_ => ()
				}
			}
			score.push(line_score);
		}
	}
	score.sort();
	println!("score: {:?}", &score[(score.len()-1)/2]);

	Ok(())
}
