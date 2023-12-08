/* Written by Juan Pablo Gutiérrez */
/*
use rand::Rng;
use std::cmp::Ordering;
use std::io; */

use rand::Rng;
use std::fmt::Write;
use std::io;

const WIDTH: usize = 12; // 2 more to account for |
const HEIGHT: usize = 40;

fn main() {
    let mut screen: [[&str; WIDTH]; HEIGHT] = [[""; WIDTH]; HEIGHT];

    create_screen(&mut screen);
    // Game Loop
    loop {
        create_tetromino(screen);
        processInput();
        display_screen(&screen);
    }
}

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

fn create_tetromino(screen: [[&str; WIDTH]; HEIGHT]) {
    let x: u32 = random_tetronimo();

    let mut tetromino: String = String::new();

    if x == 1 {
        write!(
            tetromino,
            " 
[][][][]
            "
        )
        .unwrap();
        println!("{}", tetromino);
    } else if x == 2 {
        write!(
            tetromino,
            "
[][][] 
    []
            "
        )
        .unwrap();
        println!("{}", tetromino);
    } else if x == 3 {
        write!(
            tetromino,
            "
[][]
[][]
            "
        )
        .unwrap();
        println!("{}", tetromino);
    } else if x == 4 {
        write!(
            tetromino,
            "
[][]
  [][]
            "
        )
        .unwrap();
        println!("{}", tetromino);
    } else if x == 5 {
        write!(
            tetromino,
            "
[][][] 
  []
            "
        )
        .unwrap();
        println!("{}", tetromino);
    }
}

fn processInput() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
}

fn random_tetronimo() -> u32 {
    return rand::thread_rng().gen_range(1..=5);
}