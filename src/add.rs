pub fn adder(a: u32, b: u32) -> u32 {
	let mut i = 0;
	let mut result = 0;
	let mut carry = 0;
	let digit_finder = a ^ b;

	while i < 32 {
		let place = 1 << i;
		let digit = carry ^ (digit_finder & place);
		carry = (((a & b) | (a & carry) | (b & carry)) & place) << 1;
		result ^= digit;
		i += 1;
	}

	result
}

pub fn test_add() {
	println!("{} + {} = {}", 1, 2, adder(1, 2));
	println!("{} + {} = {}", 1, 2, adder(2, 1));
	println!("{} + {} = {}", 0, 0, adder(0, 0));
	println!("{} + {} = {}", 3, 4, adder(3, 4));
	println!("{} + {} = {}", 51, 89, adder(51, 89));
	println!("{} + {} = {}", 1, 1, adder(1, 1));
	println!("{} + {} = {}", 1, 0, adder(1, 0));
	println!("{} + {} = {}", 69, 0, adder(69, 0));
	println!("{} + {} = {}", 69, 2, adder(69, 2));
	println!("{} + {} = {}", u32::MAX, 0, adder(u32::MAX, 0));
	println!("{} + {} = {}", u32::MAX, 1, adder(u32::MAX, 1));
	println!("{} + {} = {}", u32::MAX, 1, adder(u32::MAX, 32));
}