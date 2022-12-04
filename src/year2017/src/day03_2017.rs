pub struct Day03_2017();

use std::collections::HashMap;

const UPDATE: &'static [(i32, i32)] = &[ (1, 0), (0, -1), (-1, 0), (0, 1) ]; 
const COUNT_UPDATE: &'static [i32] = &[ 0, 1, 0, 1 ]; 

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
struct Point {
	x: i32,
	y: i32
}

impl Day03_2017 {
	fn neighbours(p: Point, spiral: &HashMap<Point, i32>) -> i32 {
		let mut sum = 0;
		for x in -1..2 {
			for y in -1..2 {
				let np = Point {x: p.x + x, y: p.y + y};
				if spiral.contains_key(&np) {
					sum += spiral[&np];
				}
			}
		}

		return sum;
	}
}

impl aoc::Solution for Day03_2017 {

	fn part_one(&self, input: &Vec<String>) -> aoc::Output {
		let mut x: i32 = 0;
		let mut y: i32 = 0;
		let mut value = 1;
		let mut count = 1;
		let target: i32 = input[0].parse().expect("number expected");

		loop {
			for j in 0..4 {
				for _i in 0..count {
					if value == target {
						return aoc::Output::I32(x.abs() + y.abs());
					}

					x += UPDATE[j].0;
					y += UPDATE[j].1;
					value += 1;
				}

				count += COUNT_UPDATE[j];
			}
		}
	}

	fn part_two(&self, input: &Vec<String>) -> aoc::Output {
		let mut spiral: HashMap<Point, i32> = HashMap::new();
		let target: i32 = input[0].parse().expect("number expected");

		let mut p = Point {x: 0, y: 0};
		spiral.insert(p, 1);

		let mut count = 1;
		loop {
			for j in 0..4 {
				for _i in 0..count {
					let value = Day03_2017::neighbours(p, &spiral);
					if value > target {
						return aoc::Output::I32(value);
					}

					spiral.insert(p, value);
					p.x += UPDATE[j].0;
					p.y += UPDATE[j].1;
				}

				count += COUNT_UPDATE[j];
			}
		}
	}
}
