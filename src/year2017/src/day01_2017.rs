pub struct Day01_2017();

impl Day01_2017 {

	fn count(input: &str, add: usize) -> i32 {
		let mut sum = 0;
		for i in 0..input.len() {
			if input.chars().nth(i) == input.chars().nth((i + add) % input.len()) {
				sum += input.chars().nth(i).expect("OK") as i32 - '0' as i32;
			}
		}

		return sum;
	}
}

impl aoc::Solution for Day01_2017 {

	fn part_one(&self, input: &Vec<String>) -> aoc::Output {
		return aoc::Output::I32(Day01_2017::count(&input[0], 1));
	}

	fn part_two(&self, input: &Vec<String>) -> aoc::Output {
		return aoc::Output::I32(Day01_2017::count(&input[0], input[0].len() / 2));
	}
}
