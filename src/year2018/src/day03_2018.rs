pub struct Day03_2018();

use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
struct Point {
	x: i32,
	y: i32
}

struct Fabric {
	id: i32,
	point: Point,
	width: i32,
	height: i32
}

impl Day03_2018 {
	fn load(input: &Vec<String>) -> Vec<Fabric> {
		let mut fabrics: Vec<Fabric> = vec![];
		for line in input {
			let parts: Vec<&str> = line.split(' ').collect();
			let pt = parts[2].replace(":", "");
			let point: Vec<&str> = pt.split(',').collect();
			let size: Vec<&str> = parts[3].split('x').collect();
			let f = Fabric {
				id: parts[0].replace("#", "").parse::<i32>().expect("number expected"),
				point: {
					Point {
						x: point[0].parse::<i32>().expect("number expected"),
						y: point[1].parse::<i32>().expect("number expected")
					}
				},
				width: size[0].parse::<i32>().expect("number expected"),
				height: size[1].parse::<i32>().expect("number expected")
			};

			fabrics.push(f);
		}

		return fabrics;
	}
}

impl aoc::Solution for Day03_2018 {

	fn part_one(&self, input: &Vec<String>) -> aoc::Output {
		let fabrics = Day03_2018::load(input);

		let mut points: HashMap<Point, i32> = HashMap::new(); 
		for f in fabrics {
			for x in f.point.x..f.point.x + f.width {
				for y in f.point.y..f.point.y + f.height {
					let np = Point { x: x, y: y };
					let count = points.entry(np).or_insert(0);
					*count += 1; 
				}
			}
		}

		return aoc::Output::I32(points.iter().map(|x| if x.1 > &1 { 1 } else { 0 }).sum());
	}

	fn part_two(&self, input: &Vec<String>) -> aoc::Output {
		let fabrics = Day03_2018::load(input);

		let mut ids: HashSet<i32> = HashSet::new();
		let mut points: HashMap<Point, HashSet<i32>> = HashMap::new(); 
		for f in fabrics {
			ids.insert(f.id);
			for x in f.point.x..f.point.x + f.width {
				for y in f.point.y..f.point.y + f.height {
					let np = Point { x: x, y: y };
					if !points.contains_key(&np) {
						points.insert(np, HashSet::new());
					}

					points.get_mut(&np).map(|val| val.insert(f.id));
				}
			}
		}

		for pids in points.values() {
			if pids.len() > 1 {
				for id in pids {
					ids.remove(id);
				}
			}
		}

		return aoc::Output::I32(*ids.iter().next().expect("OK"));
	}
}
