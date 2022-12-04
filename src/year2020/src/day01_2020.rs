pub struct Day01_2020();

impl aoc::Solution for Day01_2020 {

	fn part_one(&self, input: &Vec<String>) -> aoc::Output {
		let x: Vec<i32> = input.iter().map(|s| s.parse().expect("parse error")).collect();
		for a in &x {
			for b in &x {
				if 2020 == a + b {
					return aoc::Output::I32(a * b);
				}
			}
		}

		return aoc::Output::I32(-1);
	}

	fn part_two(&self, input: &Vec<String>) -> aoc::Output {
		let x: Vec<i32> = input.iter().map(|s| s.parse().expect("parse error")).collect();
		for a in &x {
			for b in &x {
				for c in &x {
					if 2020 == a + b + c {
						return aoc::Output::I32(a * b * c);
					}
				}
			}
		}

		return aoc::Output::I32(-1);
	}
}
