use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

// const INPUT: &str = "./input/test.txt";
const INPUT: &str = "./input/input.txt";

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn is_small(cave: &str) -> bool {
	cave.chars().next().unwrap().is_lowercase()
}

fn explore_1<'a>(
	cave_system: &'a HashMap<&'a str, Vec<&'a str>>,
	cave: &'a str,
	breadcrumbs: HashSet<&'a str>,
) -> u32 {
	if cave.eq("end") {
		return 1;
	}
	let mut count = 0;
	for next_cave in cave_system.get(cave).unwrap() {
		let mut new_bc = breadcrumbs.clone();
		if is_small(next_cave) {
			if breadcrumbs.contains(next_cave) {
				continue;
			} else {
				new_bc.insert(*next_cave);
			}
		}
		count += explore_1(cave_system, next_cave, new_bc);
	}
	return count;
}

fn explore_2<'a>(
	cave_system: &'a HashMap<&'a str, Vec<&'a str>>,
	cave: &'a str,
	breadcrumbs: HashSet<&'a str>,
	double_back: bool,
) -> u32 {
	if cave.eq("end") {
		return 1;
	}
	let mut count = 0;
	for next_cave in cave_system.get(cave).unwrap() {
		let mut new_bc = breadcrumbs.clone();
		let mut revisit_cave = double_back.clone();
		if is_small(next_cave) {
			if breadcrumbs.contains(next_cave) {
				if !double_back || next_cave == &"start"{
					continue;
				}else{
					revisit_cave = false;
				}
			} else {
				new_bc.insert(next_cave);
			}
		}
		count += explore_2(cave_system, next_cave, new_bc, revisit_cave);
	}
	return count;
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
	let mut cave_system: HashMap<&str, Vec<&str>> = HashMap::new();
	for line in input.lines() {
		let (from, to) = line.split_once("-").unwrap();
		cave_system.entry(from).or_default().push(to);
		cave_system.entry(to).or_default().push(from);
	}
	println!("{:?}", cave_system);
	let mut breadcrumbs = HashSet::new();
	breadcrumbs.insert("start");
	let routes_count = explore_1(&cave_system, "start", breadcrumbs);
	println!("{:?}", routes_count);
	Ok(())
}

fn part2(input: &str) -> Result<()> {
	let mut cave_system: HashMap<&str, Vec<&str>> = HashMap::new();
	for line in input.lines() {
		let (from, to) = line.split_once("-").unwrap();
		cave_system.entry(from).or_default().push(to);
		cave_system.entry(to).or_default().push(from);
	}
	println!("{:?}", cave_system);
	let mut breadcrumbs = HashSet::new();
	breadcrumbs.insert("start");
	let routes_count = explore_2(&cave_system, "start", breadcrumbs, true);
	println!("{:?}", routes_count);
	Ok(())
}
