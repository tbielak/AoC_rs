pub struct Day02_2018();

use std::collections::HashMap;

impl Day02_2018 {

	fn diff(s1: &String, s2: &String) -> String {
		let mut x = String::from("");
		for i in 0..s1.len() {
			if s1.chars().nth(i) == s2.chars().nth(i) {
				x += &s1.chars().nth(i).expect("OK").to_string();
			}
		}


		return x;
	}
}

impl aoc::Solution for Day02_2018 {

	fn part_one(&self, input: &Vec<String>) -> aoc::Output {
		let mut two = 0;
		let mut three = 0;
		for s in input {
			let mut charcount: HashMap<char, i32> = HashMap::new();
			for c in s.chars() {
				let count = charcount.entry(c).or_insert(0);
				*count += 1;
			}

			if charcount.values().any(|&v| v == 2) {
				two += 1;
			}

			if charcount.values().any(|&v| v == 3) {
				three += 1;
			}
		}

		return aoc::Output::I32(two * three);
	}

	fn part_two(&self, input: &Vec<String>) -> aoc::Output {

		for i in 0..input.len() {
			for j in i + 1..input.len() {
				let x = Day02_2018::diff(&input[i], &input[j]);
				if x.len() == input[i].len() - 1 {
					return aoc::Output::STRING(x);
				}
			}
		}

		return aoc::Output::STRING("?".to_string());
	}
}
