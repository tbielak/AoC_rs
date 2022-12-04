pub struct Day03_2015();

use std::collections::HashSet;

impl aoc::Solution for Day03_2015 {

	fn part_one(&self, input: &Vec<String>) -> aoc::Output {
		let mut x = 0;
		let mut y = 0;
		let mut visited: HashSet<(i32, i32)> = HashSet::new();
		visited.insert((x, y));

		for c in input[0].chars() {
			if '^' == c { y -= 1; }
			if 'v' == c { y += 1; }
			if '>' == c { x -= 1; }
			if '<' == c { x += 1; }
			visited.insert((x, y));
		}

		return aoc::Output::USIZE(visited.len());
	}

	fn part_two(&self, input: &Vec<String>) -> aoc::Output {
		let mut x: [i32; 2] = [0; 2];
		let mut y: [i32; 2] = [0; 2];
		let mut visited: HashSet<(i32, i32)> = HashSet::new();
		visited.insert((x[0], y[0]));

		let mut i = 0;
		for c in input[0].chars() {
			if '^' == c { y[i] -= 1; }
			if 'v' == c { y[i] += 1; }
			if '>' == c { x[i] -= 1; }
			if '<' == c { x[i] += 1; }
			visited.insert((x[i], y[i]));
			i ^= 1;
		}

		return aoc::Output::USIZE(visited.len());
	}
}
