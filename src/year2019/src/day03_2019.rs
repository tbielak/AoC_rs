pub struct Day03_2019();

use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
struct Point {
	x: i32,
	y: i32
}

struct Wire {
	path: Vec<(char, i32)>,
	points: HashMap<Point, i32>
}

impl Wire {
	fn load(input: &str) -> Vec<(char, i32)> {
		let mut path = Vec::new();
		let data: Vec<&str> = input.split(",").collect(); 
		for s in data {
			let direction: char = s.chars().nth(0).expect("char expected");
			let distance: i32 = s[1..].parse::<i32>().expect("number expected");
			path.push((direction, distance));
		}

		return path;
	}

	fn twists_and_turns(&mut self) {
		let mut step = 0;
		let mut x = 0;	
		let mut y = 0;
		const DIRECTIONS:&'static str = "RULD"; 
		const TURNS: &'static [(i32, i32)] = &[ (1, 0), (0, -1), (-1, 0), (0, 1) ];

		for (direction, distance) in &self.path {
			for _i in 0..*distance {
				let d = DIRECTIONS.find(*direction).expect("correct direction expected");
				x += TURNS[d].0;
				y += TURNS[d].1;

				step += 1;
				self.points.insert(Point {x: x, y: y}, step);
			}
		}
	}

	fn distance_to_closest(&self, other: &Wire) -> i32 {
		let mut distance = i32::MAX;
		for p in self.points.keys() {
			if other.points.contains_key(p) {
				distance = distance.min(p.x.abs() + p.y.abs());
			}
		}

		return distance;
	}

	fn fewest_steps(&self, other: &Wire) -> i32 {
		let mut steps = i32::MAX;
		for (p, s1) in &self.points {
			if other.points.contains_key(p) {
				steps = steps.min(s1 + other.points[p]);
			}
		}

		return steps;
	}
}

impl aoc::Solution for Day03_2019 {

	fn both_parts(&self, input: &Vec<String>) -> aoc::Output {
		let mut w1 = Wire { path: Wire::load(&input[0]), points: HashMap::new() };
		let mut w2 = Wire { path: Wire::load(&input[1]), points: HashMap::new() };
		w1.twists_and_turns();
		w2.twists_and_turns();
		return aoc::Output::PairI32(w1.distance_to_closest(&w2), w1.fewest_steps(&w2));
	}
}
