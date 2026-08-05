mod add;
mod gray_code;
mod mult;
use crate::{add::test_add, gray_code::test_gray_code, mult::test_mult};

fn main() {
    test_add();
    test_mult();
    test_gray_code();
}
