use std::io::BufRead;

pub fn main() {
	// 37  36  35  34  33  32 31
	// 38  17  16  15  14  13 30 // 5
	// 39  18   5   4   3  12 29 // 3
	// 40  19   6   1   2  11 28 // 1
	// 41  20   7   8   9  10 27
	// 42  21  22  23  24  25 26
	// 43  44  45  46  47  48 49

	// rings of size 1 + 2n
	// move right to new ring: prev + max of ring
	// move right:
	//   bottom: prev + 1
	//   top: prev - 1
	// move left:
	//   bottom: prev - 1
	//   top: prev + 1
	//
	// max num in ring: m = (1 + 2n)^2
	// to find which ring x is in:
	// n = ceil?((sqrt(x) - 1) / 2)
	// then: find (x, y) coords and compute manhattan
	// distance
	// 15 -> (0, 2)
	// 13 -> (2, 2)
	// 25 -> (2, -2) == (2, 2)
	// @SPAGHETTI
	let stdin = std::io::stdin();
	let mut stdin_lock = stdin.lock();
	let mut buf = String::new();
	stdin_lock.read_line(&mut buf).expect("Missing input");
	let new_buf_len = buf.len() - 1;
	buf.truncate(new_buf_len);

	let num: i64 = buf.parse().expect("Failed to parse input");
	let ring = (((num as f64).sqrt() - 1.0) / 2.0).ceil() as i64;
	let ring_size = 2 * ring + 1;
	let ring_max = (1 + 2 * ring) * (1 + 2 * ring);
	let ring_min = (1 + 2 * (ring - 1)) * (1 + 2 * (ring - 1)) + 1;
	let ring_half = ((ring_max - ring_min) / 2) + ring_min;
	println!(
		"ring: {}, ring_size: {}, ring_max: {}, ring_min: {}, ring_half: {}",
		ring,
		ring_size,
		ring_max,
		ring_min,
		ring_half,
	);
	let num = if (num > ring_half) {
		num - ring_half + ring_min - 1
	} else {
		num
	};
	let x = if (num == ring_half + 1) || (num <= ring_min + ring_size - 2) {
		ring_size - (ring_size + 1) / 2
	} else {
		ring_half - num - (ring_size) / 2
	};
	let y = if num == ring_half + 1 {
		-(ring_size - (ring_size + 1) / 2)
	} else if num <= ring_min + ring_size - 2 {
		(ring_min + ring_size - 2) - num - (ring_size) / 2
	} else {
		ring_size - (ring_size + 1) / 2
	};
	println!("{}", x.abs() + y.abs());
}
