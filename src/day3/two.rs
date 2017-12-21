use std::collections::HashMap;
use std::io::BufRead;

type Coords = (i64, i64);

fn fill(field: &mut HashMap<Coords, u64>, pos: Coords) -> u64 {
	let neighbors = [
		(pos.0 - 1, pos.1 + 1),
		(pos.0, pos.1 + 1),
		(pos.0 + 1, pos.1 + 1),
		(pos.0 - 1, pos.1),
		(pos.0 + 1, pos.1),
		(pos.0 - 1, pos.1 - 1),
		(pos.0, pos.1 - 1),
		(pos.0 + 1, pos.1 - 1),
	];
	let sum = neighbors
		.iter()
		.map(|p| field.get(p))
		.map(|o| o.map(|v| *v).unwrap_or(0))
		.fold(0, |acc, n| acc + n);
	field.insert(pos, sum);
	sum
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum SpiralDirection {
	Up,
	Left,
	Down,
	Right,
}

impl SpiralDirection {
	fn next(self, pos: Coords) -> (Coords, Self) {
		let (x, y) = pos;
		let (xp, yp) = self.get_offset();
		let r = (x + xp, y + yp);
		if (r.0 - 1 == -r.1 && self == SpiralDirection::Right) {
			(r, self.cycle())
		} else if (r.0.abs() == r.1.abs() && self != SpiralDirection::Right) {
			(r, self.cycle())
		} else {
			(r, self)
		}
	}

	fn get_offset(self) -> Coords {
		match self {
			SpiralDirection::Up => (0, 1),
			SpiralDirection::Left => (-1, 0),
			SpiralDirection::Down => (0, -1),
			SpiralDirection::Right => (1, 0),
		}
	}

	fn cycle(self) -> Self {
		match self {
			SpiralDirection::Up => SpiralDirection::Left,
			SpiralDirection::Left => SpiralDirection::Down,
			SpiralDirection::Down => SpiralDirection::Right,
			SpiralDirection::Right => SpiralDirection::Up,
		}
	}
}

struct SpiralCoordIter {
	pos: Coords,
	dir: SpiralDirection,
}

impl Iterator for SpiralCoordIter {
	type Item = Coords;

	fn next(&mut self) -> Option<Self::Item> {
		let (pos, dir) = self.dir.next(self.pos);
		self.pos = pos;
		self.dir = dir;
		Some(self.pos)
	}
}

fn first_greater_than(n: u64) -> u64 {
	let mut field = HashMap::new();
	field.insert((0, 0), 1);
	let mut spiral_coord_iter = SpiralCoordIter {
		pos: (0, 0),
		dir: SpiralDirection::Right,
	};
	for pos in spiral_coord_iter {
		let v = fill(&mut field, pos);
		if v > n {
			return v;
		};
	}
	unreachable!();
}

pub fn main() {
	let stdin = std::io::stdin();
	let mut stdin_lock = stdin.lock();
	let mut buf = String::new();
	stdin_lock.read_line(&mut buf).expect("Missing input");
	let new_buf_len = buf.len() - 1;
	buf.truncate(new_buf_len);

	let num: u64 = buf.parse().expect("Failed to parse input");
	println!("{}", first_greater_than(num));
}
