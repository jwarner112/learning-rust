fn main() {
    let x:f32 = 32.0; // 32b Float
    let y:f64 = 64.0; // 64b Float
    let z     = 64.0; // 64b Float (default)

    print!("\n");
    print!("In Rust, floats follow the IEEE-754 standard.\n");
    print!("The default float size is 64b (a double).\n");
    print!("Both 32b and 64b sizes are valid.\n");
    print!("    x (32b): {}\n", x);
    print!("    y (64b): {}\n", y);
    print!("    z (64b): {}\n", z);
    print!("\n");
}
