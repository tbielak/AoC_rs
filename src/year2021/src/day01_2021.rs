pub struct Day01_2021();

impl aoc::Solution for Day01_2021 {

	fn part_one(&self, input: &Vec<String>) -> aoc::Output {
		let x: Vec<i32> = input.iter().map(|s| s.parse().expect("parse error")).collect(); 
		let mut c = 0;
		for i in 1..x.len() {
			if x[i] > x[i - 1] {
				c += 1;
			}
		}

		return aoc::Output::I32(c);
	}

	fn part_two(&self, input: &Vec<String>) -> aoc::Output {
		let x: Vec<i32> = input.iter().map(|s| s.parse().expect("parse error")).collect(); 
		let mut c = 0;
		for i in 3..x.len() {
			if x[i] + x[i - 1] + x[i - 2] > x[i - 1] + x[i - 2] + x[i - 3] {
				c += 1;
			}
		}
	
		return aoc::Output::I32(c);
	}
}
