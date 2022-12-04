pub struct Day02_2019();

use std::collections::HashMap;

struct IntcodeVM {
	memory: HashMap<i32, i32>,
	ip: i32
}

impl IntcodeVM {
	fn parse(input: &str) -> HashMap<i32, i32> {
		let mut memory: HashMap<i32, i32> = HashMap::new();
		let data: Vec<&str> = input.split(",").collect();
		for i in 0..data.len() {
			memory.insert(i as i32, data[i].parse::<i32>().expect("number expected"));
		}
		return memory;
	}

	fn patch(&mut self, address: i32, value: i32) {
		self.memory.insert(address, value);
	}

	fn peek(&self, address: i32) -> i32 {
		return self.memory[&address];
	}

	fn fetch(&mut self) -> i32 {
		self.ip += 1;
		return self.memory[&self.memory[&(self.ip - 1)]];
	}

	fn store(&mut self, value: i32) {
		self.ip += 1;
		self.memory.insert(self.memory[&(self.ip - 1)], value);
	}

	fn run(&mut self) {
		loop {
			self.ip += 1;
			match self.memory[&(self.ip - 1)] {
				1 => {
					let x = self.fetch();
					let y = self.fetch();
					self.store(x + y);
				},
				2 => {
					let x = self.fetch();
					let y = self.fetch();
					self.store(x * y);
				},
				99 => return,
				_ => panic!("unsupported opcode")
			}
		}
	}
}

impl aoc::Solution for Day02_2019 {

	fn part_one(&self, input: &Vec<String>) -> aoc::Output {
		let mut vm = IntcodeVM { memory: IntcodeVM::parse(&input[0]), ip: 0 };
		vm.patch(1, 12);
		vm.patch(2, 2);
		vm.run();
		return aoc::Output::I32(vm.peek(0));
	}

	fn part_two(&self, input: &Vec<String>) -> aoc::Output {
		let program = IntcodeVM::parse(&input[0]);
		for noun in 0..101 {
			for verb in 0..101 {
				let mut vm = IntcodeVM { memory: program.clone(), ip: 0 };
				vm.patch(1, noun);
				vm.patch(2, verb);
				vm.run();
				if 19690720 == vm.peek(0) {
					return aoc::Output::I32(100 * noun + verb);
				}
			}
		}

		return aoc::Output::I32(-1);
	}
}
