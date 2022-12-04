use std::env;
use std::collections::HashMap;

pub mod day01_2018;
pub mod day02_2018;
pub mod day03_2018;

fn main() {

	let repository = aoc::Repository {
		year: 2018,
		days: HashMap::from([
			(1, aoc::OneDay {
				title: "--- Day 1: Chronal Calibration ---".to_string(),
				solved_parts: 2,
				solutions: Vec::from([
					aoc::OneSolution { description: "".to_string(), solution: Box::new(day01_2018::Day01_2018{}) }
				])
			}),
			(2, aoc::OneDay {
				title: "--- Day 2: Inventory Management System ---".to_string(),
				solved_parts: 2,
				solutions: Vec::from([
					aoc::OneSolution { description: "".to_string(), solution: Box::new(day02_2018::Day02_2018{}) }
				])
			}),
			(3, aoc::OneDay {
				title: "--- Day 3: No Matter How You Slice It ---".to_string(),
				solved_parts: 2,
				solutions: Vec::from([
					aoc::OneSolution { description: "".to_string(), solution: Box::new(day03_2018::Day03_2018{}) }
				])
			}),
		])
	};

	aoc::Engine::run(repository, env::args().collect());
}
