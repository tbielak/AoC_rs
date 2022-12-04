pub struct Day02_2016();

impl Day02_2016 {

	fn press(keymap: [&'static str; 4], digit: String) -> String {
		let mut curr = '5';
		const DIRECTIONS:&'static str = "UDLR";
		for d in digit.chars() { 
			let key = DIRECTIONS.find(d).expect("correct direction expected");
			let idx = curr as i32 - '0' as i32;
			curr = keymap[key].chars().nth(idx as usize).expect("OK");
		}

		return if curr <= '9' { curr.to_string() } else { char::from_u32(curr as u32 + 7).expect("OK").to_string() };
	}

	fn enter_code(keymap: [&'static str; 4], input: &Vec<String>) -> String {
		let mut code = String::from(""); 
		for digit in input.iter() {
			code += &Day02_2016::press(keymap, digit.to_string());

		}

		return code;
	}
}

impl aoc::Solution for Day02_2016 {

	fn part_one(&self, input: &Vec<String>) -> aoc::Output {
		return aoc::Output::STRING(
			Day02_2016::enter_code(["0123123456", "0456789789", "0112445778", "0233566899"], input));
	}

	fn part_two(&self, input: &Vec<String>) -> aoc::Output {
		return aoc::Output::STRING(
			Day02_2016::enter_code(["0121452349678;", "036785:;<9:=<=", "0122355678::;=", "0134467899;<<="], input));
	}
}
