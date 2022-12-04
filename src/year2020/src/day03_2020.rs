pub struct Day03_2020();

impl aoc::Solution for Day03_2020 {

	fn part_one(&self, input: &Vec<String>) -> aoc::Output {
		let mut x = 0;
		let mut c = 0;
		let width = input[0].len();
		for s in input {
			if s.chars().nth(x).expect("OK") == '#' {
				c += 1;
			}
			x = (x + 3) % width;
		}

		return aoc::Output::I32(c);
	}

	fn part_two(&self, input: &Vec<String>) -> aoc::Output {
		let mut m: i64 = 1;
		let width = input[0].len();
		let height = input.len();

		const SLOPES: &'static [(usize, usize)] = &[ (1, 1), (3, 1), (5, 1), (7, 1), (1, 2) ]; 
		for (sx, sy) in SLOPES {
			let mut x = 0;
			let mut y = 0;
			let mut c = 0;
			loop {
				if input[y].chars().nth(x).expect("OK") == '#' {
					c += 1;
				}

				x += sx;
				y += sy;
				x %= width;
				if y >= height {
					break;
				}
			}

			m *= c;
		}

		return aoc::Output::I64(m);
	}
}
