fn main() {
    let arr:[u32;8] = [224, 120, 191, 512, 783, 829, 900, 921];
    print!("\n");
    print!("In Rust, arrays are of a fixed and static length.\n");
    print!("Elements can be accessed with typical index notation.\n");
    print!("    arr = [224, 120, 191, 512, 783, 829, 900, 921]\n");
    print!("    arr[0]: {}\n", arr[0]);
    print!("    arr[2]: {}\n", arr[2]);
    print!("    arr[5]: {}\n", arr[5]);
    print!("    arr[7]: {}\n", arr[7]);
    print!("If accessing an invalid index, your program will panic.\n");
    print!("\n");
}
