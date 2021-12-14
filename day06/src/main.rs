use std::error::Error;
use std::fs;
// const INPUT: &str = "./input/test.txt";
const INPUT: &str = "./input/input.txt";

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
struct LanternFish {
	timer: i32,
}
impl LanternFish {
	fn step(&mut self) -> bool {
		let new_time = self.timer - 1;
		if new_time < 0 {
			self.timer = 6;
			return true;
		} else {
			self.timer = new_time;
			return false;
		}
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

// The dumb way (just simulate it)
fn part1(input: &str) -> Result<()> {
	let mut fish_list: Vec<LanternFish> = input
		.split(",")
		.map(|x| LanternFish {
			timer: x.parse().unwrap(),
		})
		.collect();
	for day in 0..80 {
		println!("{}", day);

		let pop = fish_list.len();
		for f in 0..pop {
			if fish_list[f].step() {
				fish_list.push(LanternFish { timer: 8 })
			}
		}
	}
	println!("{:?}", fish_list.len());
	Ok(())
}

// Only keep track of the number of fish not every fish
fn part2(input: &str) -> Result<()> {
	let fish_list: Vec<u8> = input.split(",").map(|x| x.parse().unwrap()).collect();
	let mut timeline = vec![0u64; 9];
	for f in fish_list {
		timeline[f as usize] += 1;
	}
	for _ in 0..256 {
		// println!("{:?}", timeline);
		let wrap = timeline[0];
		for jj in 0..8 {
			timeline[jj] = timeline[jj + 1];
		}
		timeline[8] = wrap;
		timeline[6] += wrap;
		// println!("{:?}", timeline);
		// println!("");
	}
	println!("{:?}", timeline.iter().fold(0, |x, a| a + x));
	Ok(())
}
