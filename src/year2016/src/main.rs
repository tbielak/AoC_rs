use std::env;
use std::collections::HashMap;

pub mod day01_2016;
pub mod day02_2016;
pub mod day03_2016;

fn main() {

	let repository = aoc::Repository {
		year: 2016,
		days: HashMap::from([
			(1, aoc::OneDay {
				title: "--- Day 1: No Time for a Taxicab ---".to_string(),
				solved_parts: 2,
				solutions: Vec::from([
					aoc::OneSolution { description: "".to_string(), solution: Box::new(day01_2016::Day01_2016{}) }
				])
			}),
			(2, aoc::OneDay {
				title: "--- Day 2: Bathroom Security ---".to_string(),
				solved_parts: 2,
				solutions: Vec::from([
					aoc::OneSolution { description: "".to_string(), solution: Box::new(day02_2016::Day02_2016{}) }
				])
			}),
			(3, aoc::OneDay {
				title: "--- Day 3: Squares With Three Sides ---".to_string(),
				solved_parts: 2,
				solutions: Vec::from([
					aoc::OneSolution { description: "".to_string(), solution: Box::new(day03_2016::Day03_2016{}) }
				])
			}),
		])
	};

	aoc::Engine::run(repository, env::args().collect());
}
