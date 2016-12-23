extern crate rand;

use std::io;
use std::io::Write;
use std::cmp::Ordering;

use rand::Rng;

const LO:u32 = 0;
const HI:u32 = 99;


fn main() {
    print!("\n");
    print!("Guessing Game!\n");
    print!("    Valid range (incl): ({a}, {b})\n", a=LO, b=HI);
    print!("==================================\n");

    let num = rand::thread_rng().gen_range(LO, HI);

    loop {
        let mut guess = String::new();

        print!("Please input your guess: ");
        #[allow(unused_must_use)]
        io::stdout().flush().expect("Could not flush stdout");
        io::stdin().read_line(&mut guess).expect("Could not read input");

        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&num) {
            Ordering::Less    => { print!("Too small!\n"); },
            Ordering::Greater => { print!("Too large!\n"); },
            Ordering::Equal   => {
                print!("You win!\n");
                break;
            }
        }
    }

    print!("\n");
}
