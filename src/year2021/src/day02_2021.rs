pub struct Day02_2021();

impl Day02_2021 {
	fn load(input: &Vec<String>) -> Vec<(char, i32)> {
		let mut course: Vec<(char, i32)> = vec![];
		for line in input {
			let s: Vec<&str> = line.split(" ").collect();
			let command: char = s[0].chars().nth(0).expect("char expected");
			let units: i32 = s[1].parse::<i32>().expect("number expected"); 
			course.push((command, units));
		}

		return course;
	}
}

impl aoc::Solution for Day02_2021 {

	fn part_one(&self, input: &Vec<String>) -> aoc::Output {
		let mut position = 0;
		let mut depth = 0;
		for (command, units) in Day02_2021::load(input) {
			match command {
				'd' => depth += units,
				'u' => depth -= units,
				_ => position += units
			}
		}

		return aoc::Output::I32(position * depth);
	}

	fn part_two(&self, input: &Vec<String>) -> aoc::Output {
		let mut position = 0;
		let mut depth = 0;
		let mut aim = 0;
		for (command, units) in Day02_2021::load(input) {
			match command {
				'd' => aim += units,
				'u' => aim -= units,
				_ => {
					position += units;
					depth += aim * units;
				}
			}
		}

		return aoc::Output::I32(position * depth);
	}
}
