use std::collections::HashSet;
use std::io::BufRead;
use std::str::SplitWhitespace;

pub fn main() {
	let stdin = std::io::stdin();
	let mut stdin_lock = stdin.lock();
	let mut buf = String::new();
	let mut valid_count = 0u64;
	while let Ok(n) = stdin_lock.read_line(&mut buf) {
		if n == 0 {
			break;
		}

		let inc = {
			let mut split = buf.split_whitespace();
			|split: SplitWhitespace| -> u8 {
				let mut seen: HashSet<&str> = HashSet::new();
				for w in split {
					if let Some(_) = seen.get(w) {
						return 0;
					}
					seen.insert(w);
				}
				return 1;
			}(split)
		};

		valid_count = valid_count + inc as u64;

		buf.clear();
	}
	println!("{}", valid_count);
}
