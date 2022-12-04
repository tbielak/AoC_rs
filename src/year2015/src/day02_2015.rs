pub struct Day02_2015();

impl Day02_2015 {

	fn load(input: &Vec<String>) -> Vec<(usize, usize, usize)> {
		let mut items: Vec<(usize, usize, usize)> = vec![];
		for s in input.iter()
		{
			let x: Vec<&str> = s.split("x").collect();
			let t: (usize, usize, usize) = (x[0].parse().unwrap(), x[1].parse().unwrap(), x[2].parse().unwrap());
			items.push(t);
		}
		return items;
	}
}

impl aoc::Solution for Day02_2015 {

	fn part_one(&self, input: &Vec<String>) -> aoc::Output {
		let items = Day02_2015::load(input);
		let mut area: usize = 0;
		let mut s: [usize; 3] = [0; 3];
		for i in items.iter()
		{
			let (l, w, h) = i;
			s[0] = l * w;
			s[1] = w * h;
			s[2] = h * l;
			s.sort();
			area += (s[0] + s[1] + s[2]) * 2 + s[0];
		}

		return aoc::Output::USIZE(area);
	}

	fn part_two(&self, input: &Vec<String>) -> aoc::Output {
		let items = Day02_2015::load(input);
		let mut ribbon: usize = 0;
		let mut s: [usize; 3] = [0; 3];
		for i in items.iter()
		{
			s[0] = i.0;
			s[1] = i.1;
			s[2] = i.2;
			s.sort();
			ribbon += 2 * (s[0] + s[1]) + s[0] * s[1] * s[2];
		}
	
		return aoc::Output::USIZE(ribbon);
	}
}
