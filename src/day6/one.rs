use std::collections::HashSet;
use std::io::BufRead;

pub fn main() {
	let stdin = std::io::stdin();
	let mut stdin_lock = stdin.lock();
	let mut buf = String::new();
	if stdin_lock.read_line(&mut buf).expect("missing line") <= 0 {
		panic!("empty line");
	};
	let mut banks: Vec<u64> = buf.split_whitespace()
		.map(|s| s.parse().expect("failed to parse number"))
		.collect();
	let mut seen: HashSet<Vec<u64>> = HashSet::new();
	let mut count = 0;
	while seen.get(&banks).is_none() {
		seen.insert(banks.clone());
		// find biggest
		let mut biggest_idx = 0;
		let mut biggest = 0;
		for i in 0..banks.len() {
			if banks[i] > biggest {
				biggest = banks[i];
				biggest_idx = i;
			};
		}

		// empty
		let mut n = biggest;
		banks[biggest_idx] = 0;

		// redistribute
		let mut idx = (biggest_idx + 1) % banks.len();
		while n > 0 {
			banks[idx] += 1;
			idx = (idx + 1) % banks.len();
			n -= 1;
		}

		count += 1;
	}
	println!("{}", count);
}
