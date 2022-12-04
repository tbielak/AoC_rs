use std::env;
use std::collections::HashMap;

pub mod day01_2020;
pub mod day02_2020;
pub mod day03_2020;

fn main() {

	let repository = aoc::Repository {
		year: 2020,
		days: HashMap::from([
			(1, aoc::OneDay {
				title: "--- Day 1: Report Repair ---".to_string(),
				solved_parts: 2,
				solutions: Vec::from([
					aoc::OneSolution { description: "".to_string(), solution: Box::new(day01_2020::Day01_2020{}) }
				])
			}),
			(2, aoc::OneDay {
				title: "--- Day 2: Password Philosophy ---".to_string(),
				solved_parts: 2,
				solutions: Vec::from([
					aoc::OneSolution { description: "".to_string(), solution: Box::new(day02_2020::Day02_2020{}) }
				])
			}),
			(3, aoc::OneDay {
				title: "--- Day 3: Toboggan Trajectory ---".to_string(),
				solved_parts: 2,
				solutions: Vec::from([
					aoc::OneSolution { description: "".to_string(), solution: Box::new(day03_2020::Day03_2020{}) }
				])
			}),
		])
	};

	aoc::Engine::run(repository, env::args().collect());
}
