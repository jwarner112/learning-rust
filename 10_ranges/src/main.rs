
fn main() {
    print!("\n");

    print!("    Iter over range:\n");
    print!("      ");
    #[allow(unused_parens)]
    for n in (5..10) { print!("{} ", n); }
    print!("\n\n");

    print!("    Iter over reversed range:\n");
    print!("      ");
    for n in (5..10).rev() { print!("{} ", n); }
    print!("\n\n");

    print!("    Iter over inclusive range:\n");
    print!("      ");
    print!("can't use yet; see issue #28237");
    // for n in (5...10) { println!("{}", n); }
    print!("\n\n");

    // Observation: Cannot re-use a Range?
}
