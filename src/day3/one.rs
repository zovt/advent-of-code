pub fn main() {
	// 17  16  15  14  13 // 5
	// 18   5   4   3  12 // 3
	// 19   6   1   2  11 // 1
	// 20   7   8   9  10
	// 21  22  23---> ...

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
}
