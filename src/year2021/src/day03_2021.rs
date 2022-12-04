pub struct Day03_2021();

impl Day03_2021 {
	fn find_rating(x: &Vec<String>, xor_mask: u32) -> i32 {
		let mut consider: Vec<bool> = vec![];
		for _i in 0..x.len() {
			consider.push(true);
		}

		loop {
			for i in 0..x[0].len() {
				let mut zeros = 0;
				let mut ones = 0;
				for j in 0..x.len() {
					if consider[j] {
						if x[j].chars().nth(i) == Some('1') {
							ones += 1;
						}
						else {
							zeros += 1;
						}
					}
				}

				let keep = char::from_u32((if ones >= zeros { 1 } else { 0 }) ^ xor_mask + '0' as u32);
				for j in 0..x.len() {
					if consider[j] && x[j].chars().nth(i) == keep {
						consider[j] = false;
					}
				}

				let mut c = 0;
				for j in 0..consider.len() {
					if consider[j] {
						c += 1;
					}
				}

				if c == 1 {
					for j in 0..x.len() {
						if consider[j] {
							return i32::from_str_radix(&x[j], 2).unwrap();
						}
					}
				}
			}
		}
	}
}


impl aoc::Solution for Day03_2021 {

	fn part_one(&self, input: &Vec<String>) -> aoc::Output {
		let mut gamma_bin = String::from(""); 
		let mut epsilon_bin = String::from(""); 
		for i in 0..input[0].len() {
			let zeros: i32 = input.iter().map(|s| if s.chars().nth(i) == Some('0') { 1 } else { 0 }).sum(); 
			let ones = input.len() as i32 - zeros;
			gamma_bin += if zeros <= ones { "1" } else { "0" };
			epsilon_bin += if zeros > ones { "1" } else { "0" };
		}

		let gamma_rate = i32::from_str_radix(&gamma_bin, 2).unwrap();
		let epsilon_rate = i32::from_str_radix(&epsilon_bin, 2).unwrap();
		return aoc::Output::I32(gamma_rate * epsilon_rate);
	}

	fn part_two(&self, input: &Vec<String>) -> aoc::Output {
		let oxygen_rate = Day03_2021::find_rating(input, 0);
		let co2_rate = Day03_2021::find_rating(input, 1);
		return aoc::Output::I32(oxygen_rate * co2_rate);
	}

}
