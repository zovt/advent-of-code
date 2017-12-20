pub fn main() {
	let num_string = std::env::args().nth(1).unwrap();
	let num_bytes = num_string.as_str().as_bytes();
	let len = num_bytes.len();
	let mut sum: u64 = 0;
	for i in 0..len {
		let (fst, snd) = (num_bytes[i] as u8, num_bytes[(i + 1) % len] as u8);
		if fst == snd {
			sum = sum + (fst - ('0' as u8)) as u64;
		}
	}
	println!("{}", sum);
}
