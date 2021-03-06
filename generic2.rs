use std::fmt::Display;
use std::cmp::PartialOrd;

fn compare_and_display<T: Display, U: Display + PartialOrd>(statement: T, num_1: U, num_2: U) {
    println!("{}! Is {} greater than {}? {}", statement, num_1, num_2, num_1 > num_2);
}

fn main() {
    compare_and_display("Listen up!", 9, 8);
}

// Same Code As Above But With 'where'

// use std::cmp::PartialOrd;
// use std::fmt::Display;

// fn compare_and_display<T, U>(statement: T, num_1: U, num_2: U)
// where
//     T: Display,
//     U: Display + PartialOrd,
// {
//     println!("{}! Is {} greater than {}? {}", statement, num_1, num_2, num_1 > num_2);
// }

// fn main() {
//     compare_and_display("Listen up!", 9, 8);
// }
