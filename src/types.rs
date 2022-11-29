/*
Primitive Types
--------------------------

* integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
* floats: f32, f64
* boolean: (bool)
* characters: (char)
* tuples
* arrays

*/

// Rust is a statically typed language, which means that it must know the Types
// of variables at compile time, however, the compiler can usually infer what
// type we want to use based on the value and how we use it.

pub fn run() {
    // Default is "i32"
    let x = 1;

    // Default is "f64"
    let y = 2.23;

    // Add an explicit type
    let z: i64 = 234134125;

    // Find MAX size
    println!("Max i32 is {}", std::i32::MAX);
    println!("Max i64 is {}", std::i64::MAX);
}
