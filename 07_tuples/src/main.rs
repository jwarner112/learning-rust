fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    print!("\n");
    print!("In Rust, tuples are an arbitrary collection of values.\n");
    print!("Tuple elements can be referenced by index:\n");
    print!("    tup.0: {}\n", tup.0);
    print!("    tup.1: {}\n", tup.1);
    print!("    tup.2: {}\n", tup.2);
    print!("Tuple elements can also be found by deconstruction:\n");
    print!("    x: {}\n", x);
    print!("    y: {}\n", y);
    print!("    z: {}\n", z);
    print!("\n");
}
