pub struct Day03_2016();

impl Day03_2016 {

	fn load(input: &Vec<String>) -> Vec<[i32; 3]> {
		let mut items: Vec<[i32; 3]> = vec![];
		for s in input.iter()
		{
			let mut x: [i32; 3] = [0; 3];
			for i in 0..3 {
				x[i] = *&s[i*5..i*5+5].trim().parse::<i32>().expect("invalid number");
			}
		
			items.push(x);
		}

		return items;
	}

	fn is_triangle(mut x: [i32; 3]) -> i32 {
		x.sort();
		return if x[0] + x[1] > x[2] { 1  } else { 0 };
	}
}

impl aoc::Solution for Day03_2016 {

	fn part_one(&self, input: &Vec<String>) -> aoc::Output {
		return aoc::Output::I32(Day03_2016::load(input).iter().map(|&x| Day03_2016::is_triangle(x)).sum());
	}

	fn part_two(&self, input: &Vec<String>) -> aoc::Output {
		let data = Day03_2016::load(input);

		let mut count = 0;
		let col_sets = data.len() / 3;
		for i in 0..col_sets {
			for j in 0..3 {
				let mut x: [i32; 3] = [0; 3];
				for k in 0..3 {
					x[k] = data[i * 3 + k][j];
					count += Day03_2016::is_triangle(x);
				}
			}
		}

		return aoc::Output::I32(count);
	}
}
