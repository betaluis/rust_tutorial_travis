// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you neet to
// modify or own string

pub fn run() {
    let hello = "hello";

    let hi = String::from("Hi");

    println!("{}", hello);
    println!("{}", hi);
}
