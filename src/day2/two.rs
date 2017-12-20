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

		nums.sort();
		sum = sum +
			|nums: &Vec<u64>| -> u64 {
				for i in 0..nums.len() {
					for j in (0..nums.len()).rev() {
						if i == j {
							continue;
						}
						if nums[j] % nums[i] == 0 {
							println!("{}, {}", nums[j], nums[i]);
							return nums[j] / nums[i];
						}
					}
				}
				0
			}(&nums);

		nums.clear();
		line.clear();
	}

	println!("{}", sum);
}
