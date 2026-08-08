mod add;
mod bool_eval;
mod gray_code;
mod mult;
mod powerset;
mod sat;
mod truth_table;
mod curve;

use crate::{
    add::test_add, bool_eval::test_eval_formula, gray_code::test_gray_code, mult::test_mult,
    powerset::print_powesets, sat::test_sat, truth_table::show_tables, curve::test_map
};

fn main() {
    test_add();
    test_mult();
    test_gray_code();
    test_eval_formula();
    show_tables();
    test_sat();
    println!();
    print_powesets();
	test_map();
}
