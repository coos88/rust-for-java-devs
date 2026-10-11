// Ex05: traits, generics
// cargo run -p phase1 --bin ex05_traits_generics
//
// 1. trait Describe { fn describe(&self) -> String; } plus a default method `shout`.
// 2. Implement it for two structs.
// 3. fn print_all<T: Describe>(items: &[T]) and fn print_mixed(items: &[Box<dyn Describe>]).
//    Which one can hold both struct types at once? Why?
//    Hint: annotate the Vec's type (Vec<Box<dyn Describe>>), or the second Box won't coerce.

fn main() {
    todo!()
}
