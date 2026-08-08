mod add;
mod bool_eval;
mod gray_code;
mod mult;
mod truth_table;
use crate::{
    add::test_add, bool_eval::test_eval_formula, gray_code::test_gray_code, mult::test_mult,
    truth_table::print_truth_table,
};

fn main() {
    test_add();
    test_mult();
    test_gray_code();
    test_eval_formula();
    print_truth_table("AB&C|");
}
