pub struct Day01_2018();

use std::collections::HashSet;

impl aoc::Solution for Day01_2018 {

	fn part_one(&self, input: &Vec<String>) -> aoc::Output {
		return aoc::Output::I32(input.iter().map(|x| x.parse::<i32>().expect("number expected")).sum());
	}

	fn part_two(&self, input: &Vec<String>) -> aoc::Output {
		let mut known: HashSet<i32> = HashSet::new(); 
		let mut freq = 0;

		loop {
			for s in input {
				freq += s.parse::<i32>().expect("number expected");
				if known.contains(&freq) {
					return aoc::Output::I32(freq);
				}

				known.insert(freq);
			}
		}
	}
}
