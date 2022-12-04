pub struct Day01_2019();

impl Day01_2019 {

	fn fuel_req(mut v: i32) -> i32 {
		let mut sum = 0;
		while v > 0 {
			v = (v / 3 - 2).max(0);
			sum += v;
		}

		return sum;
	}
}

impl aoc::Solution for Day01_2019 {

	fn part_one(&self, input: &Vec<String>) -> aoc::Output {
		return aoc::Output::I32(input.iter().map(|x| x.parse::<i32>().expect("number expected") / 3 - 2).sum());
	}

	fn part_two(&self, input: &Vec<String>) -> aoc::Output {
		return aoc::Output::I32(input.iter().map(
			|x| Day01_2019::fuel_req(x.parse::<i32>().expect("number expected"))).sum());
	}
}
