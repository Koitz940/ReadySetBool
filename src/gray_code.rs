pub fn gray_code(n: u32) -> u32 {
    n ^ (n >> 1)
}

fn test1graycode(n: u32, expected: u32) {
    assert!(
        gray_code(n) == expected,
        "Mistake calculating gray code of {}, espected {}, got {}",
        n,
        expected,
        gray_code(n)
    );
}

pub fn test_gray_code() {
    test1graycode(0, 0);
    test1graycode(1, 1);
    test1graycode(2, 3);
    test1graycode(3, 2);
    test1graycode(4, 6);
    test1graycode(5, 7);
    test1graycode(6, 5);
    test1graycode(7, 4);
    test1graycode(8, 12);
}
