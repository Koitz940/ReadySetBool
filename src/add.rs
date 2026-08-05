pub fn adder(a: u32, b: u32) -> u32 {
    let mut n = a;
    let mut m = b;
    let mut carry;

    while m != 0 {
        carry = m & n;
        n ^= m;
        m = carry << 1;
    }

    n
}

fn test1add(a: u32, b: u32) {
    assert!(
        a + b == adder(a, b),
        "Mistake doing {} + {}, expected {}, got {}",
        a,
        b,
        a + b,
        adder(a, b)
    );
}

pub fn test_add() {
    test1add(1, 2);
    test1add(2, 1);
    test1add(0, 0);
    test1add(3, 4);
    test1add(51, 89);
    test1add(1, 1);
    test1add(1, 0);
    test1add(69, 0);
    test1add(69, 2);
    test1add(std::u32::MAX, 0);
}


/* pub fn adder2(a: u32, b: u32) -> u32 {
    let mut n = a;
    let mut m = b;
    let mut carry;
    let mut i = 0;

    while i != 32 {
        carry = m & n;
        n ^= m;
        m = carry << 1;
        i += 1;
    }

    n
} */


/* 
fn test2add(a: u32, b: u32) {
    assert!(
        a + b == adder(a, b),
        "Mistake doing {} + {}, expected {}, got {}",
        a,
        b,
        a + b,
        adder(a, b)
    );
} */



/* pub fn test2_add() {
    test2add(1, 2);
    test2add(2, 1);
    test2add(0, 0);
    test2add(3, 4);
    test2add(51, 89);
    test2add(1, 1);
    test2add(1, 0);
    test2add(69, 0);
    test2add(69, 2);
    test2add(std::u32::MAX, 0);
}

pub fn time2_add() {
	for _ in 0..100000 {
		test2_add();
	}
}

pub fn time1_add() {
	for _ in 0..100000 {
		test1_add();
	}
}
 */