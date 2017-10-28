extern crate example_crate_errors;

use example_crate_errors::safe_divide_from_stdin;

fn main() {
    let result = safe_divide_from_stdin().unwrap();
    println!("{}", result);
}