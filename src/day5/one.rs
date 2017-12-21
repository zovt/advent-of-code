use std::io::BufRead;

pub fn main() {
	let stdin = std::io::stdin();
	let stdin_lock = stdin.lock();
	let mut stack: Vec<i64> = stdin_lock
		.lines()
		.map(|l| l.expect("Line error").parse().expect("Failed to parse int"))
		.collect();
	let mut stack_idx: usize = 0;
	let mut count = 0;
	while stack_idx < stack.len() {
		let offset = stack[stack_idx];
		stack[stack_idx] += 1;
		stack_idx = (stack_idx as i64 + offset) as usize;
		count += 1;
	}
	println!("{}", count);
}
