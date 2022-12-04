use std::env;
use std::collections::HashMap;

pub mod day01_2021;
pub mod day02_2021;
pub mod day03_2021;

fn main() {

	let repository = aoc::Repository {
		year: 2021,
		days: HashMap::from([
			(1, aoc::OneDay {
				title: "--- Day 1: Sonar Sweep ---".to_string(),
				solved_parts: 2,
				solutions: Vec::from([
					aoc::OneSolution { description: "".to_string(), solution: Box::new(day01_2021::Day01_2021{}) }
				])
			}),
			(2, aoc::OneDay {
				title: "--- Day 2: Dive! ---".to_string(),
				solved_parts: 2,
				solutions: Vec::from([
					aoc::OneSolution { description: "".to_string(), solution: Box::new(day02_2021::Day02_2021{}) }
				])
			}),
			(3, aoc::OneDay {
				title: "--- Day 3: Binary Diagnostic ---".to_string(),
				solved_parts: 2,
				solutions: Vec::from([
					aoc::OneSolution { description: "".to_string(), solution: Box::new(day03_2021::Day03_2021{}) }
				])
			}),
		])
	};

	aoc::Engine::run(repository, env::args().collect());
}
