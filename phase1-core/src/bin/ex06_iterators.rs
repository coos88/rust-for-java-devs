// Ex06: collections, iterators, closures
// cargo run -p phase1 --bin ex06_iterators
//
// No explicit loops:
// 1. Vec of lengths of words longer than 3 chars.
// 2. HashMap<char, usize>: number of words per first letter.
//    Hint: w.chars().next() gives an Option<char>.
//    .fold(HashMap::new(), |mut m, w| { *m.entry(..).or_insert(0) += 1; m })
// 3. Longest word as Option<&str>. Hint: max_by_key, then .copied().

use std::collections::HashMap;

fn main() {
    let words = ["rust", "java", "borrow", "cargo", "jvm", "trait", "go"];
    let _ = (words, HashMap::<char, usize>::new());
    todo!()
}
