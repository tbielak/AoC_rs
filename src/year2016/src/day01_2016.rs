pub struct Day01_2016();

use std::collections::HashSet;

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
struct Position {
	x: i32,
	y: i32,
}

impl Position {
	fn walk(&mut self, face: i32, steps: i32) {
		const UPDATE: &'static [(i32, i32)] = &[ (0, -1), (1, 0), (0, 1), (-1, 0) ];
		self.x += UPDATE[face as usize].0 * steps;
		self.y += UPDATE[face as usize].1 * steps;
	}

	fn distance(self) -> i32 {
		return self.x.abs() + self.y.abs();
	}
}

impl Day01_2016 {

	fn load(input: &str) -> Vec<(i32, i32)> {
		let mut route: Vec<(i32, i32)> = vec![];
		let no_spaces = input.replace(" ", "");
		let items: Vec<&str> = no_spaces.split(",").collect();
		for cmd in items.iter()
		{
			let steps: i32 = cmd[1..].parse::<i32>().unwrap();
			let direction: i32 = if cmd.chars().nth(0) == Some('R') { 1 } else { -1 };
			route.push((direction, steps));
		}

		return route;
	}
}

impl aoc::Solution for Day01_2016 {

	fn part_one(&self, input: &Vec<String>) -> aoc::Output {
		let mut face: i32 = 0;
		let mut position = Position {x: 0, y: 0};
		for cmd in Day01_2016::load(&input[0]).iter() {
			let (direction, steps) = cmd;
			face = (face + direction) & 3;
			position.walk(face, *steps);
		}
	
		return aoc::Output::I32(position.distance());
	}

	fn part_two(&self, input: &Vec<String>) -> aoc::Output {
		let mut face: i32 = 0;
		let mut position = Position {x: 0, y: 0};
		let mut visited: HashSet<Position> = HashSet::new(); 
		visited.insert(position);

		for cmd in Day01_2016::load(&input[0]).iter() {
			let (direction, steps) = cmd;
			face = (face + direction) & 3;
			for _i in 0..*steps {
				position.walk(face, 1);
				if visited.contains(&position) {
					return aoc::Output::I32(position.distance());
				}

				visited.insert(position);
			}
		}

		return aoc::Output::I32(-1);
	}
}
