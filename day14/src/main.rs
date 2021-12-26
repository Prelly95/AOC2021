use std::collections::HashMap;
use std::error::Error;
use std::fs;

const INPUT: &str = "./input/test.txt";
// const INPUT: &str = "./input/input.txt";

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
	let input = fs::read_to_string(INPUT).expect("Something went wrong reading the file");

	println!("\npart 1:");
	part1(&input)?;
	println!("\npart 2:");
	// part2(&input)?;
	Ok(())
}

fn part1(input: &str) -> Result<()> {
	let (template_str, instruction_strings) = input.split_once("\n\n").unwrap();
	let mut template = String::from(template_str);
	let mut instructions = HashMap::new();
	for line in instruction_strings.lines() {
		let (pair, insert) = line.split_once(" -> ").unwrap();
		instructions.insert(pair, insert);
	}

	for ii in 0..40 {
		let mut inserts = Vec::new();
		println!("{}\npolymerising...", ii);
		for ins in instructions.iter() {
			for ii in 1..template.len() {
				let comp = &template[ii-1..=ii];
				if ins.0.eq(&comp) {
					inserts.push((ii, ins.1));
				}
			}
		}
		println!("sorting...");
		inserts.sort();
		println!("interting...");
		for jj in inserts.iter().rev() {
			let left = &template[0..jj.0];
			let right = &template[jj.0..];
			template = format!("{}{}{}", left, jj.1, right);
		}
		println!("done\n");//step {}  - Polymer length: {}", ii, template.len());
	}

	let mut letter_count: HashMap<char, u32> = HashMap::new();
	for c in template.chars() {
		if let Some(count) = letter_count.get_mut(&c) {
			*count += 1;
		} else {
			letter_count.insert(c.clone(), 1);
		}
	}

	let max = letter_count.iter().map(|x| x.1).max().unwrap();
	let min = letter_count.iter().map(|x| x.1).min().unwrap();
	println!("{:?}", max - min);
	Ok(())
}

// fn part2(input: &str) -> Result<()> {
// 	todo!();
// 	Ok(())
// }
