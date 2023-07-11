/* Written by Juan Pablo Gutiérrez */

use rand::Rng;
use std::cmp::Ordering;
use std::io;

const width: u32 = 20;
const height: u32 = 60;

fn main() {
    let score: u32 = 0;

    let a: [i32; 4] = [1, 2, 3, 4];
   /*  let w : usize = width.parse();
    let b: [[u32; width]; height] = [[2123], [123]]; */
    // Game Loop
    loop {}
}

fn screen() {
    
    let x : usize;
    for i in 0..=height {
        println!("");
        print!(" | ");
        for j in 0..=width {
            print!(" . ");
        }
        print!(" | ");
    }
}

fn guessing() {
    println!("Welcome to the guessing game!!");

    let secret_num: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Guess the number");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
