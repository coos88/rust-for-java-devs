// Ex02: ownership, moves, borrowing
// cargo run -p phase1 --bin ex02_ownership
//
// Uncomment one block at a time. Make it compile WITHOUT .clone().
// Before fixing, explain to yourself why it failed.

fn takes(s: String) -> usize { s.len() }
fn borrows(s: &String) -> usize { s.len() }
fn appends(s: &mut String) { s.push('!'); }

fn main() {
    // A: use after move
    // let s = String::from("hi");
    // takes(s);
    // println!("{s}");

    // B: mutate through a shared borrow
    // let s = String::from("hi");
    // appends(&s);

    // C: two mutable borrows alive at once
    // let mut s = String::from("hi");
    // let a = &mut s;
    // let b = &mut s;
    // a.push('a'); b.push('b');

    // D: return a reference to a local. Fix by changing the return type.
    // fn make() -> &String { let s = String::from("x"); &s }

    let _ = (takes, borrows, appends);
}
