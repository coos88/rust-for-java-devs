// Ex01: variables, types, control flow, functions
// cargo run -p phase1 --bin ex01_basics
//
// 1. fizzbuzz(n) -> String for 1..=15, print each line.
// 2. Use `match` on a tuple (n % 3, n % 5), not if/else.
// 3. Try reassigning a non-`mut` variable. Read the error.

fn fizzbuzz(n: u32) -> String {
    todo!()
}

fn main() {
    for n in 1..=15 {
        println!("{}", fizzbuzz(n));
    }
}
