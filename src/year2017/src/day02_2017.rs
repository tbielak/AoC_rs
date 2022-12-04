pub struct Day02_2017();

impl Day02_2017 {

	fn load(input: &Vec<String>) -> Vec<Vec<i32>> {
		let mut items: Vec<Vec<i32>> = vec![];
		for s in input.iter()
		{
			let mut row: Vec<i32> = vec![];
			let x: Vec<&str> = s.split("\t").collect(); 
			for i in 0..x.len() {
				row.push(x[i].parse::<i32>().unwrap());
			}

			items.push(row);
		}

		return items;
	}
}

impl aoc::Solution for Day02_2017 {

	fn part_one(&self, input: &Vec<String>) -> aoc::Output {
		return aoc::Output::I32(Day02_2017::load(input).iter().map(
			|row| row.iter().max().expect("OK") - row.iter().min().expect("OK")).sum());
	}

	fn part_two(&self, input: &Vec<String>) -> aoc::Output {
		let data = Day02_2017::load(input);

		let mut sum = 0;
		for row in data {
			for i in 0..row.len() {
				for j in i + 1..row.len() {
					let mut a = row[i];
					let mut b = row[j];

					if a < b {
						(b, a) = (a, b);
					}

					if (a % b) == 0 {
						sum += a / b;
						break;
					}
				}
			}
		}

		return aoc::Output::I32(sum);
	}
}
