use std::env;
use std::collections::HashMap;

pub mod day01_2015;
pub mod day02_2015;
pub mod day03_2015;

fn main() {

	let repository = aoc::Repository {
		year: 2015,
		days: HashMap::from([
			(1, aoc::OneDay {
				title: "--- Day 1: Not Quite Lisp ---".to_string(),
				solved_parts: 2,
				solutions: Vec::from([
					aoc::OneSolution { description: "".to_string(), solution: Box::new(day01_2015::Day01_2015{}) }
				])
			}),
			(2, aoc::OneDay {
				title: "--- Day 2: I Was Told There Would Be No Math ---".to_string(),
				solved_parts: 2,
				solutions: Vec::from([
					aoc::OneSolution { description: "".to_string(), solution: Box::new(day02_2015::Day02_2015{}) }
				])
			}),
			(3, aoc::OneDay {
				title: "--- Day 3: Perfectly Spherical Houses in a Vacuum ---".to_string(),
				solved_parts: 2,
				solutions: Vec::from([
					aoc::OneSolution { description: "".to_string(), solution: Box::new(day03_2015::Day03_2015{}) }
				])
			}),
		])
	};

	aoc::Engine::run(repository, env::args().collect());
}
