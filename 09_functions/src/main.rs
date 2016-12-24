fn main() {
    print!("\n");
    print!("In Rust, you have statements, and expressions.\n");
    print!("A statement returns no value.\n");
    print!("    - Function definitions are a statement.\n");
    print!("    - Assignments are a statement, so multi-assignments don't work.\n");
    print!("An expression returns a value.\n");
    print!("    - Function calls are expressions.\n");
    print!("    - Blocks ({{}}) are expressions. This means...\n");
    print!("        let x = 5;          // Statement\n");
    print!("        let y = {{\n");
    print!("            let z = x * x;\n");
    print!("            z + 1           // (Note the lack of ending semicolon)\n");
    print!("        }}                   // This block was an expression, evaluated to: 26\n");
    print!("\n");
}
