use crate::add::adder;

fn dumb_adder(a: u32, b: u32) -> u32 {
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
}

pub fn multiplier(a: u32, b: u32) -> u32 {
    let mut n = a;
    let mut m = b;
    let mut result = 0;
    let mut i = 0;
    while i != 32 {
        if m & 1 != 0 {
            result = dumb_adder(result, n);
        }
        n <<= 1;
        m >>= 1;
        i += 1;
    }
    result
}

fn test1mult(a: u32, b: u32) {
    assert!(
        a * b == multiplier(a, b),
        "Mistake doing {} * {}, expected {}, got {}",
        a,
        b,
        a * b,
        multiplier(a, b)
    );
}

pub fn test_mult() {
    test1mult(0, 0);
    test1mult(0, 1);
    test1mult(1, 1);
    test1mult(23432, 1);
    test1mult(1, 23432);
    test1mult(2, 2);
    test1mult(100, 100);
    test1mult(51, 49);
    test1mult(2423, 23423);
    test1mult(16, 32);
    test1mult(42, 42);
}

pub fn good_multiplier(a: u32, b: u32) -> u32 {
    let mut n = a;
    let mut m = b;
    let mut result = 0;
    while m != 0 {
        if m & 1 != 0 {
            result = adder(result, n);
        }
        n <<= 1;
        m >>= 1;
    }
    result
}

/* fn test2mult(a: u32, b: u32) {
    assert!(
        a * b == good_multiplier(a, b),
        "Mistake doing {} * {}, expected {}, got {}",
        a,
        b,
        a * b,
        good_multiplier(a, b)
    );
} */

/* pub fn test2_mult() {
    test2mult(0, 0);
    test2mult(0, 1);
    test2mult(1, 1);
    test2mult(23432, 1);
    test2mult(1, 23432);
    test2mult(2, 2);
    test2mult(100, 100);
    test2mult(51, 49);
    test2mult(2423, 23423);
    test2mult(16, 32);
    test2mult(42, 42);
}

fn time_mult() {
    for _ in 0..100000 {
        test_mult(2423, 23423);
    }
}

fn time2_mult() {
    for _ in 0..100000 {
        test2_mult(2423, 23423);
    }
}
 */
