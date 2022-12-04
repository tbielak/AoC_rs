use std::env;
use std::collections::HashMap;

pub mod day01_2017;
pub mod day02_2017;
pub mod day03_2017;

fn main() {

	let repository = aoc::Repository {
		year: 2017,
		days: HashMap::from([
			(1, aoc::OneDay {
				title: "--- Day 1: Inverse Captcha ---".to_string(),
				solved_parts: 2,
				solutions: Vec::from([
					aoc::OneSolution { description: "".to_string(), solution: Box::new(day01_2017::Day01_2017{}) }
				])
			}),
			(2, aoc::OneDay {
				title: "--- Day 2: Corruption Checksum ---".to_string(),
				solved_parts: 2,
				solutions: Vec::from([
					aoc::OneSolution { description: "".to_string(), solution: Box::new(day02_2017::Day02_2017{}) }
				])
			}),
			(3, aoc::OneDay {
				title: "--- Day 3: Spiral Memory ---".to_string(),
				solved_parts: 2,
				solutions: Vec::from([
					aoc::OneSolution { description: "".to_string(), solution: Box::new(day03_2017::Day03_2017{}) }
				])
			}),
		])
	};

	aoc::Engine::run(repository, env::args().collect());
}
