/* Written by Juan Pablo Gutiérrez */
/*
use rand::Rng;
use std::cmp::Ordering;
use std::io; */

const WIDTH: usize = 10;
const HEIGHT: usize = 40;

fn main() {
    let mut screen: [[&str; WIDTH]; HEIGHT] = [[""; WIDTH]; HEIGHT];

    create_screen(&mut screen);
    display_screen(&screen);
    // Game Loop
/*     loop {}
 */}

fn create_screen(screen: &mut [[&str; WIDTH]; HEIGHT]) {
    for i in 0..HEIGHT {
        screen[i][0] = "| ";
        for j in 1..WIDTH - 1 {
            screen[i][j] = " . ";
        }
        screen[i][WIDTH - 1] = " |";
    }
}

fn display_screen(screen: &[[&str; WIDTH]; HEIGHT]) {
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            print!("{}", screen[i][j]);
        }
        println!();
    }
}

/*
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
 */
