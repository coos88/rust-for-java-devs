// Ex04: Option, Result, ?
// cargo run -p phase1 --bin ex04_errors
//
// 1. parse_age(&str) -> Result<u8, String>: Err if not a number or > 150.
// 2. first_even(&[i32]) -> Option<i32>.
// 3. A fn returning Result that calls parse_age twice using `?`.
// 4. In main, .unwrap() an Err from parse_age. Run it, check the exit code (`echo $?`),
//    then run again with RUST_BACKTRACE=1.
// No .unwrap() outside main.

fn main() {
    todo!()
}
