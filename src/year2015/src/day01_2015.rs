pub struct Day01_2015();

impl aoc::Solution for Day01_2015 {

	fn part_one(&self, input: &Vec<String>) -> aoc::Output {
		return aoc::Output::USIZE(input[0].matches('(').count() - input[0].matches(')').count());
	}

	fn part_two(&self, input: &Vec<String>) -> aoc::Output {
		let mut floor = 0;
		for (i, c) in input[0].chars().enumerate() { 
			if '(' == c { floor += 1; }
			if ')' == c { floor -= 1; }
			if -1 == floor { return aoc::Output::USIZE(i + 1); }
		}

		return aoc::Output::NONE;
	}
}
