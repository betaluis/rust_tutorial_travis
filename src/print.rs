pub fn run() {
    println!("Hello from print.rs");

    // Basic formatting
    println!("Number: {}", 2);
    println!("{} is from {}", "Luis", "Houston");

    // Positional formatting
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Luis", "Houston", "code"
    );

    // Named arguments
    println!(
        "{name} likes to play {activity}",
        name = "John",
        activity = "soccer"
    );

    // Placeholder traits
    println!("Binary: {:b} Hex {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}
