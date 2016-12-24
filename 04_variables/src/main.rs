fn show(x:u32, y:u32){
    print!("\n");
    print!("x: {}\n", x);
    print!("y: {}\n", y);
}

fn show_str(x:u32, y:String){
    print!("\n");
    print!("x: {}\n", x);
    print!("y: {}\n", y);
}

fn main() {
    // Immutable variable
    let x = 10;
    // Mutable Variable
    let mut y = 5;
    show(x, y);
    // Reassign Mutable Variable
    y = 10;
    show(x, y);
    // Shadow Immutable Variable
    let x = x * 2;
    show(x, y);
    // Shadow Mutable Variable, change type (with annotation)
    let mut y:String = String::new();
    y.push_str("Now I'm a string!");
    show_str(x, y);
}
