extern crate num_cpus;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::time::Instant;

#[derive(PartialEq)]
pub enum Output {
	NONE,
	I32(i32),
	I64(i64),
	USIZE(usize),
	STRING(String),
	PairI32(i32, i32)
}

pub trait Solution {
	fn part_one(&self, _input: &Vec<String>) -> Output {
		return Output::NONE;
	}

	fn part_two(&self, _input: &Vec<String>) -> Output {
		return Output::NONE;
	}

	fn both_parts(&self, _input: &Vec<String>) -> Output {
		return Output::NONE;
	}
}

impl dyn Solution {

	fn process_results(output: Output, results: &mut Vec<String>) {
		match output {
			Output::NONE => (),
			Output::I32(v) => results.push(v.to_string()),
			Output::I64(v) => results.push(v.to_string()),
			Output::USIZE(v) => results.push(v.to_string()),
			Output::STRING(v) => results.push(v),
			Output::PairI32(v1, v2) => { results.push(v1.to_string()); results.push(v2.to_string()); }
		}
	}

	fn run(&self, input: &Vec<String>) -> (Vec<String>, Vec<f64>) {

		let mut results: Vec<String> = vec![];
		let mut exec_times: Vec<f64> = vec![];

		let t = Instant::now();
		let output = self.both_parts(input);
		let elapsed: f64 = t.elapsed().as_micros() as f64 / 1000.;

		if output != Output::NONE {
			<dyn Solution>::process_results(output, &mut results);
			exec_times.push(elapsed);
			return (results, exec_times);
		}

		let t = Instant::now();
		let output = self.part_one(input);
		let elapsed: f64 = t.elapsed().as_micros() as f64 / 1000.;
		<dyn Solution>::process_results(output, &mut results);
		exec_times.push(elapsed);

		let t = Instant::now();
		let output = self.part_two(input);
		let elapsed: f64 = t.elapsed().as_micros() as f64 / 1000.;
		if output != Output::NONE {
			<dyn Solution>::process_results(output, &mut results);
			exec_times.push(elapsed);
		}

		return (results, exec_times);
	}
}

pub struct OneSolution {
	pub description: String,
	pub solution: Box<dyn Solution>
}

pub struct OneDay {
	pub title: String,
	pub solved_parts: i32,
	pub solutions: Vec<OneSolution>
}

pub struct Repository {
	pub year: i32,
	pub days: HashMap<i32, OneDay>
} 

pub struct Options {
	colors: bool,
	help: bool,
	available: bool,
	speed: i32,
	day: i32,
	solution: i32,
	input_filename: String
}

impl Options
{
	fn parse(&mut self, args: Vec<String>) {
		for mut i in 0..args.len() {
			let x = &args[i];
			if x == "-c" {
				self.colors = false;
			}

			if x == "-h" {
				self.help = true;
			}

			if x == "-a" {
				self.available = true;
			}

			if x == "-s" {
				self.speed = 10;
				if args.len() > i + 1 {
					i += 1;
					self.speed = args[i].parse::<i32>().expect("invalid -s argument, number expected"); 
				}
			}

			if x == "-p" && args.len() > i + 1 {
				i += 1;
				let subs: Vec<&str> = args[i].split(":").collect();  
				self.day = subs[0].parse::<i32>().expect("invalid -p argument, number expected"); 
				self.solution = if subs.len() == 1 { -1 } else { subs[1].parse::<i32>().expect("invalid second -p argument, number expected") - 1 }
			}

			if x == "-i" && args.len() > i + 1 {
				i += 1;
				self.input_filename = args[i].to_string();
			}
		}
	}
}

#[link(name = "kernel32")]
extern "stdcall" {
    pub fn GetStdHandle(nStdHandle: u64) -> u64;
}

#[link(name = "kernel32")]
extern "stdcall" {
    pub fn SetConsoleMode(hConsoleHandle: u64, mode: u64) -> i32;
}

pub struct Console {
	colors: bool
}

impl Console {
	pub fn init(&mut self) {
		if !self.colors {
			return;
		}

		const STD_OUTPUT_HANDLE: i64 = -11;
		const ENABLE_PROCESSED_OUTPUT: u64 = 1;
		const ENABLE_WRAP_AT_EOL_OUTPUT: u64 = 2;
		const ENABLE_VIRTUAL_TERMINAL_PROCESSING: u64 = 4;
		unsafe
		{
			let handle = GetStdHandle(STD_OUTPUT_HANDLE as u64);
			let err = SetConsoleMode(handle, ENABLE_PROCESSED_OUTPUT |
					ENABLE_WRAP_AT_EOL_OUTPUT | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
			if err == 0 {
				self.colors = false;
			}
		} 
	}

	pub fn write_line(&self, s: String) {
		let mut out = s;
		if self.colors {
			out = out.replace("{d}", "\x1B[0m").replace("{y}", "\x1B[93m").replace("{w}", "\x1B[97m").replace("{g}", "\x1B[92m").replace("{r}", "\x1B[91m");
		}
		else {
			out = out.replace("{d}", "").replace("{y}", "").replace("{w}", "").replace("{g}", "").replace("{r}", "");
		}

		println!("{}", out);
	}
}

pub struct Engine {
}

impl Engine {
	pub fn intro(console: &Console, repository: &Repository) {
		console.write_line("{w}AdventOfCode.rs: ".to_owned() + &repository.year.to_string() + " Puzzle Solutions{d}, copyright(c) 2022 by {y}TomB{d}");
	}

	pub fn help(console: &Console) {
		console.write_line("{y}Available options:{d}".to_owned());
		console.write_line("{g}-h{d} : help (you are exploring it right now)".to_owned());
		console.write_line("{g}-c{d} : disable colorful output (useful when streaming to file)".to_owned());
		console.write_line("{g}-a{d} : show available solutions".to_owned());
		console.write_line("{g}-p <day>{d} : run solution(s) of selected <day> only (by default all available puzzle solutions are executed)".to_owned());
		console.write_line("{g}-p <day>:<n>{d} : run n-th solution of puzzle from day <day> (if it is not available, first one is executed)".to_owned());
		console.write_line("{g}-i <filename>{d} : run with <filename> as input (if not specified inputs are loaded from 'input' directory)".to_owned());
		console.write_line("{g}-s <x>{d} : execution speed testing (at least 10 times, at least <x> seconds to measure reliable execution time)".to_owned());
		console.write_line("".to_owned());
		console.write_line("{y}Warning:{d} Not all combinations of options are supported. See valid ones in examples below:".to_owned());
		console.write_line("{g}<no options provided>{d} => run everything once, using as input the appropriate files read from 'input' directory".to_owned());
		console.write_line("{g}-a{d} => show available solutions".to_owned());
		console.write_line("{g}-p 2{d} => run only the solution(s) of 2nd day puzzle".to_owned());
		console.write_line("{g}-p 4:2{d} => run 2nd solution of 4th day puzzle".to_owned());
		console.write_line("{g}-p 5 -i my_input.txt{d} => run 5th day puzzle solution with the input read from my_input.txt file".to_owned());
		console.write_line("{g}-s 10{d} => execution speed testing of all puzzles, at least 10 times for 10 seconds (it may take a while!)".to_owned());
		console.write_line("{g}-p 7 -s 5{d} => execution speed testing 7th puzzle solution(s), at least 10 times for 5 seconds".to_owned());
		console.write_line("".to_owned());
		console.write_line("Enjoy!".to_owned());
	}

	pub fn available_solutions(console: &Console, repository: &Repository) {
		console.write_line("{y}Solutions matrix:{d}".to_owned());
		console.write_line("{g}         1111111111222222{d}".to_owned());
		console.write_line("{g}1234567890123456789012345{d}  <== day".to_owned());

		let mut solved: String = "".to_string();
		let mut solutions: String = "".to_string();
		for d in 1..26 {
			if repository.days.contains_key(&d) {
				solved += &repository.days[&d].solved_parts.to_string();
				solutions += &repository.days[&d].solutions.len().to_string();
			}
			else {
				solved += "-";
				solutions += "-";
			}
		}

		console.write_line(solved + "  <== parts solved");
		console.write_line(solutions + "  <== number of solutions available");
	}

	fn read_file(file: &str) -> Result<String, std::io::Error> {
		let mut file = File::open(file)?;
		let mut data = String::new();
		return file.read_to_string(&mut data).map(|_| data);
	}

	fn dir_prefix() -> String {
		let mut up = String::from("");
		let curr_dir = std::env::current_dir().expect("correct path").display().to_string() + "\\";
		const REP: &'static [&str] = &[ "\\src\\", "\\target\\", "\\debug\\", "\\release\\" ];  
		for prefix in REP {
			if !curr_dir.find(prefix).is_none() {
				up = "..\\".to_owned() + &up;
			}
		}

		return up;
	}

	fn load_input(console: &Console, input_filename: &String, year: i32, day: i32) -> Vec<String> {
		
		let filename = if input_filename != "" { input_filename.to_string() } else { format!("{}input\\{}_{:02}.txt", Engine::dir_prefix(), year, day) };
		let mut lines: Vec<String> = vec![];
		let mut contents = String::from("");
		match Engine::read_file(&filename) {
			Ok(cts) => contents = cts,
			Err(_) => ()
		}

		if contents == "" {
			console.write_line("{r}ERROR: Cannot load input file!{d}".to_owned());
			return lines;
		}

		lines = contents.lines().map(|s| s.to_string()).collect();
		while lines.last().expect("string expected").is_empty() {
			lines.pop();
		}

		return lines;
	}

	pub fn milliseconds(total: f64) -> String {
		let mut ms = String::from("{y}");
		ms += &((((total * 10000.0) as i64) as f64) / 10000.0).to_string();
		ms += " ms{d}";
		return ms;
	}

	pub fn print_output(console: &Console, results: &(Vec<String>, Vec<f64>), count: i32) {
		for line in &results.0 {
			console.write_line(line.to_string());
		}

		let mut msg: String = if count == 0 { "Execution time = ".to_string() } else { "Average execution time = ".to_string() };
		let total: f64 = results.1.iter().sum();
		msg += &Engine::milliseconds(total);

		if results.1.len() > 1 {
			msg += " (";
			for i in 0..results.1.len() {
				msg += &Engine::milliseconds(results.1[i]);
				if i < results.1.len() - 1 {
					msg += " / ";
				}
			}
			msg += ")";
		}

		if count > 0 {
				msg += &(" (results of ".to_owned() + &count.to_string() + " executions)");
		}

		console.write_line(msg);
	}

	pub fn execute(console: &Console, repository: &Repository, print_info: bool, input_filename: &String, day: i32, solution: i32) -> (Vec<String>, Vec<f64>) {
		
		let results: Vec<String> = vec![];
		let exec_times: Vec<f64> = vec![];
	
		if !repository.days.contains_key(&day) {
			if print_info {
				console.write_line("{r}ERROR: Day ".to_owned() + &day.to_string() + &" solution(s) are not available.{d}".to_owned());
			}
			
			return (results, exec_times);
		}

		if solution >= repository.days[&day].solutions.len() as i32 {
			if print_info {
				console.write_line("{r}ERROR: Requested solution is not available.{d}".to_owned());
			}

			return (results, exec_times);
		}

		if print_info {
			console.write_line("{g}".to_owned() + &repository.days[&day].title.to_string() + &"{d}".to_owned());
		}

		let one_solution = &repository.days[&day].solutions[solution as usize];
		if one_solution.description != "" {
			let mut s: String = ": ".to_owned() + &one_solution.description;
			if !s.find("{T}").is_none() {
				s = s.replace("{T}", &(", ".to_owned() + &num_cpus::get().to_string() + " concurrent threads"));
			}
				
			if print_info {
				if repository.days[&day].solutions.len() > 1 {
					console.write_line("{g}--- Solution #".to_owned() + &(solution + 1).to_string() + &s + &"{d}".to_owned());
				} else {
					console.write_line("{g}--- ".to_owned() + &s[2..] + &"{d}".to_owned());
				}
			}
		}

		let input = Engine::load_input(console, &input_filename, repository.year, day);
		if input.len() == 0 {
			return (results, exec_times);
		}
		
		return one_solution.solution.run(&input);
	}
	
	pub fn execute_solution(console: &Console, repository: &Repository, speed: i32, input_filename: &String, day: i32, solution: i32) {
		if speed > 0 {
			let mut ok;
			let mut results;
			let mut ref_output: Vec<String> = vec![];
			let mut exec_times: Vec<Vec<f64>> = vec![];
			let mut count = 0;
			let mut first = true;
			let t = Instant::now();
			let max_time: f64 = speed as f64 * 1000.;
			loop {
				count += 1;
				results = Engine::execute(&console, &repository, first, &input_filename, day, solution);
				ok = results.0.len() > 0;
				if !ok {
					break;
				}

				if first {
					first = false;
					ref_output = results.0.clone();
					for _j in 0..results.1.len() {
						exec_times.push(vec![]);
					}
				} else {
					if ref_output != results.0 {
						console.write_line("{r}ERROR: Different results obtained in successive executions{d}".to_owned());
						break;
					}
				}

				for j in 0..results.1.len() {
					exec_times[j].push(results.1[j]);
				}

				let time_elapsed: f64 = t.elapsed().as_micros() as f64 / 1000.;
				if time_elapsed >= max_time as f64 && count >= 10 {
					let min_i = count / 10;
					let max_i = count - min_i;
					for j in 0..exec_times.len() {
						exec_times[j].sort_by(|a, b| a.partial_cmp(b).unwrap());
						let mut c = 0;
						results.1[j] = 0.;
						for i in min_i..max_i {
							results.1[j] += exec_times[j][i];
							c += 1;
						}
						results.1[j] /= c as f64;
					}
					break;
				}
			}

			if ok {
				Engine::print_output(&console, &results, count.try_into().unwrap());
			} else {
				console.write_line("{r}ERROR: Unable to run speed test{d}".to_owned());
			}
		}
		else {
			let results = Engine::execute(&console, &repository, true, &input_filename, day, solution);
			if results.0.len() > 0 {
				Engine::print_output(&console, &results, 0);
			}
		}
	}
	
	pub fn execute_day(console: &Console, repository: &Repository, speed: i32, input_filename: &String, day: i32) {
		if !repository.days.contains_key(&day) {
				console.write_line("{r}ERROR: Day ".to_owned() + &day.to_string() + &" solution(s) are not available.{d}".to_owned());
				return;
		}

		for i in 0..repository.days[&day].solutions.len() {
			Engine::execute_solution(&console, &repository, speed, &input_filename, day, i.try_into().unwrap());
		}
	}
	
	pub fn execute_all(console: &Console, repository: &Repository, speed: i32) {
		for day in 1..26 {
			if repository.days.contains_key(&day) {
				for i in 0..repository.days[&day].solutions.len() {
					Engine::execute_solution(&console, &repository, speed, &"".to_string(), day, i.try_into().unwrap());
				}
			}
		}
	}

	pub fn run(repository: Repository, args: Vec<String>) {
		
		let mut opt = Options { colors: true, help: false, available: false, speed: 0, day: -1, solution: -1, input_filename: "".to_string() };
		opt.parse(args);

		let mut console = Console { colors: opt.colors };
		console.init();
		Engine::intro(&console, &repository);

		if opt.help {
			Engine::help(&console);
			return;
		}

		if opt.available {
			Engine::available_solutions(&console, &repository);
			return;
		}

		if opt.speed > 0 {
			const REP: &'static [&str] = &[ "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten" ];  
			let mut s: String;
			if opt.speed > 0 && opt.speed < 11 {
				s = REP[opt.speed as usize - 1].to_string();
			}
			else {
				s = opt.speed.to_string();
			}

			s += " second";
			if opt.speed > 1 {
				s += "s";
			}

			console.write_line("{y}Warning:{d} In this mode ({g}-s{d}) each puzzle solution is run at least ten times and at least for ".to_owned() + &s + &".".to_owned());
			console.write_line("It may take some time to obtain all results, please be patient. 10% of the highest and the lowest time measurements".to_owned());
			console.write_line("are dropped, the average time of all remaining is printed. Repeatability of results is checked after each execution.".to_owned());
		}

		if opt.day > -1 {
			if opt.solution > -1 {
				Engine::execute_solution(&console, &repository, opt.speed, &opt.input_filename, opt.day, opt.solution);
			}
			else {
				Engine::execute_day(&console, &repository, opt.speed, &opt.input_filename, opt.day);
			}
		}
		else {
			Engine::execute_all(&console, &repository, opt.speed);
		}
	}
}
