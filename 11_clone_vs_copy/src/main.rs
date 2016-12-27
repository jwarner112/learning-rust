fn main() {
    // Example of Copy
    let stack_a:&str = "Hello, Stack!";
    let stack_b = stack_a; // "Copy" because str is hard-coded and known.
    print!("Copy =>\n");
    print!("    A: \"{a}\"\n", a=stack_a);
    print!("    B: \"{b}\"\n", b=stack_b);
    print!("\n");

    // Example of Move
    let heap_c = String::from("Hello, Heap!");
    let heap_d = heap_c; // "Move", heap_c ptr is invalid and cannot be used.
    print!("Move =>\n");
    print!("    C: (can't use, invalid)\n");
    print!("    D: \"{d}\"\n", d=heap_d);
    print!("\n");

    // Example of Clone
    let heap_e = String::from("Hello, Heap!");
    let heap_f = heap_e.clone(); // "Clone", which allocates String F to heap.
    print!("Clone =>\n");
    print!("    E: \"{e}\"\n", e=heap_e);
    print!("    F: \"{f}\"\n", f=heap_f);
    print!("\n");

    // Test: Stack-Only Tuple Copy
    let stack_g = (001u8, 003u8, 007u8, 015u8, 031u8, 063u8, 127u8, 255u8);
    let stack_h = stack_g;
    print!("Tuple Copy =>\n");
    print!("    G.7: {g} (u8)\n", g=stack_g.7);
    print!("    H.7: {h} (u8)\n", h=stack_h.7);
    print!("\n");

    // Test: Stack-Only Array Copy
    let stack_i:[u8;8] = [001u8, 003u8, 007u8, 015u8, 031u8, 063u8, 127u8, 255u8];
    let stack_j = stack_i;
    print!("Array Copy =>\n");
    print!("    I[7]: {i} (u8)\n", i=stack_i[7]);
    print!("    J[8]: {j} (u8)\n", j=stack_j[7]);
    print!("\n");

    // Test: Stack-Only Tuple Clone
    let stack_k = (001u8, 003u8, 007u8, 015u8, 031u8, 063u8, 127u8, String::from("Test"));
    let stack_l = stack_k.clone();
    print!("Tuple Copy =>\n");
    print!("    K.7: {k} (u8)\n", k=stack_k.7);
    print!("    L.7: {l} (u8)\n", l=stack_l.7);
    print!("\n");

    // Test: Stack-Only Array Clone
    let stack_m:[String;2] = [String::from("Hello"), String::from("Array!")];
    let stack_n = stack_m;
    print!("Array Copy =>\n");
    print!("    M[7]: {m} (u8)\n", m=stack_n[0]);
    print!("    N[8]: {n} (u8)\n", n=stack_n[1]);
    print!("\n");
}
