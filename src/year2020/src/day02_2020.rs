pub struct Day02_2020();

struct Item {
	min: usize,
	max: usize,
	letter: char,
	password: String
}

impl Day02_2020 {
	fn load(input: &Vec<String>) -> Vec<Item> {
		let mut data: Vec<Item> = vec![];
		for x in input {
			let s: Vec<&str> = x.split(" ").collect(); 
			let range: Vec<&str> = s[0].split("-").collect(); 
			data.push( Item {
				min: range[0].parse().expect("number expected"),
				max: range[1].parse().expect("number expected"),
				letter: s[1].chars().nth(0).expect("letter expected"),
				password: s[2].to_string()
			});
		}

		return data;
	}
}

impl aoc::Solution for Day02_2020 {

	fn part_one(&self, input: &Vec<String>) -> aoc::Output {
		let items = Day02_2020::load(input);
		let mut c = 0;
		for x in items {
			let p = x.password.matches(x.letter).count();
			if p >= x.min && p <= x.max {
				c += 1;
			}
		}

		return aoc::Output::I32(c);
	}

	fn part_two(&self, input: &Vec<String>) -> aoc::Output {
		let items = Day02_2020::load(input);
		let mut c = 0;
		for x in items {
			if (x.password.chars().nth(x.min - 1).expect("character expected") == x.letter) ^
				(x.password.chars().nth(x.max - 1).expect("character expected") == x.letter) {
				c += 1;
			}
		}

		return aoc::Output::I32(c);
	}
}
