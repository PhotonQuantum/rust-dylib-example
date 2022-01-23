use std::io::{BufRead, stdin};
use std::str::FromStr;

#[link(name = "is_odd_lib")]
extern {
    fn is_odd(x: usize) -> bool;
}

fn main() {
    println!("Please input a number:");
    let input = stdin().lock().lines().next().unwrap().unwrap();
    let num = usize::from_str(input.as_str()).unwrap();
    println!("is_odd: {}", unsafe { is_odd(num) });
}
