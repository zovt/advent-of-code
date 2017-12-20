use std::io::BufRead;

pub fn main() {
	let stdin = std::io::stdin();
	let mut stdin_lock = stdin.lock();
	let mut line = String::new();
	let mut sum = 0;
	let mut nums: Vec<u64> = Vec::new();
	while let Ok(n) = stdin_lock.read_line(&mut line) {
		if n == 0 {
			break;
		}

		line.split_whitespace()
			.map(|n| -> u64 { n.parse().unwrap() })
			.for_each(|n| nums.push(n));
		{
			let min = nums.iter().min_by(|a, b| a.cmp(b)).unwrap();
			let max = nums.iter().max_by(|a, b| a.cmp(b)).unwrap();
			sum = sum + (max - min);
		}

		nums.clear();
		line.clear();
	}

	println!("{}", sum);
}
