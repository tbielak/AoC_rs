use std::env;
use std::collections::HashMap;

pub mod day01_2019;
pub mod day02_2019;
pub mod day03_2019;

fn main() {

	let repository = aoc::Repository {
		year: 2019,
		days: HashMap::from([
			(1, aoc::OneDay {
				title: "--- Day 1: The Tyranny of the Rocket Equation ---".to_string(),
				solved_parts: 2,
				solutions: Vec::from([
					aoc::OneSolution { description: "".to_string(), solution: Box::new(day01_2019::Day01_2019{}) }
				])
			}),
			(2, aoc::OneDay {
				title: "--- Day 2: 1202 Program Alarm ---".to_string(),
				solved_parts: 2,
				solutions: Vec::from([
					aoc::OneSolution { description: "".to_string(), solution: Box::new(day02_2019::Day02_2019{}) }
				])
			}),
			(3, aoc::OneDay {
				title: "--- Day 3: Crossed Wires ---".to_string(),
				solved_parts: 2,
				solutions: Vec::from([
					aoc::OneSolution { description: "".to_string(), solution: Box::new(day03_2019::Day03_2019{}) }
				])
			}),
		])
	};

	aoc::Engine::run(repository, env::args().collect());
}
