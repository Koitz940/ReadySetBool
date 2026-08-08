use std::println;

fn powerset(set: Vec<i32>) -> Vec<Vec<i32>> {
    let mut powset = vec![];
    for i in 0..(1 << set.len()) {
        let mut new = vec![];
        for j in 0..set.len() {
            if 1 == (1 & (i >> j)) {
                new.push(set[j]);
            }
        }
        powset.push(new);
    }
    powset
}

fn print_vec(v: &Vec<i32>) {
    print!("{}", '{');
    for (i, val) in v.iter().enumerate() {
        print!("{}", *val);
        if i + 1 != v.len() {
            print!(", ");
        }
    }
    print!("{}", '}');
}

fn show1powset(set: Vec<i32>) {
    let powset = powerset(set.clone());

    print!("Powerset of ");
    print_vec(&set);
    println!("\n{}", '{');
    for v in &powset {
        print!("\t");
        print_vec(v);
        println!(",");
    }
    println!("{}\n", '}');
}

pub fn print_powesets() {
    show1powset(vec![1, 2, 3, 4]);
    show1powset(vec![]);
    show1powset(vec![1]);
    show1powset(vec![42, 69, 420, 34, 37]);
	show1powset(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
	show1powset(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
}
