/* Written by Juan Pablo Gutiérrez */

use rand::Rng;
use std::io;

const WIDTH: usize = 12; // 2 more to account for |
const HEIGHT: usize = 40;

struct Line {
    x: i32,
    y: i32,
    value: String,
}

impl Line {
    fn new(x: i32, y: i32, value: String) -> Line {
        Line {
            x: x,
            y: y,
            value: value,
        }
    }
}

struct Tetromino {
    first: Line,
    second: Line,
}

impl Tetromino {
    fn new(first: Line, second: Line) -> Tetromino {
        Tetromino {
            first: first,
            second: second,
        }
    }

    fn move_tetromino(&mut self, x_units: i32, y_units: i32) {
        self.first.x += x_units;
        self.first.y += y_units;

        self.second.x += x_units;
        self.second.y += y_units;
    }
}

fn main() {
    let mut screen: [[&str; WIDTH]; HEIGHT] = [[""; WIDTH]; HEIGHT];
    let mut tetrominoes_list: Vec<Tetromino> = Vec::new();

    let mut turn: i32 = 1;

    create_screen(&mut screen);
    create_tetronimo(&mut tetrominoes_list);
    display_screen(&screen, &tetrominoes_list);

    loop {
        process_input();
        display_screen(&screen, &tetrominoes_list);
        update_tetrominoes(&mut tetrominoes_list);
        if turn % 5 == 0 {
            println!("{}",  turn);
            create_tetronimo(&mut tetrominoes_list);
        }
        turn += 1;
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

fn display_screen(screen: &[[&str; WIDTH]; HEIGHT], tetrominoes_list: &Vec<Tetromino>) {
    for i in 0..HEIGHT {
        let mut j = 0;
        while j < WIDTH {
            let mut found_tetromino = false;

            for tetromino in tetrominoes_list {
                let skip_distance_first = (tetromino.first.value.len() / 3) as usize;
                let skip_distance_second = if tetromino.second.value.is_empty() { 0 } else { (tetromino.second.value.len() / 3) as usize };

                if (tetromino.first.x as usize == j && tetromino.first.y as usize == i)
                    || (tetromino.second.x as usize == j && tetromino.second.y as usize == i)
                {
                    if !tetromino.second.value.is_empty() && tetromino.second.value != "2" {
                        print!("{}", if tetromino.first.y == tetromino.second.y {
                            &tetromino.first.value
                        } else if tetromino.first.x == tetromino.second.x && tetromino.first.y == tetromino.second.y - 1 {
                            if i == tetromino.first.y as usize {
                                &tetromino.first.value
                            } else {
                                &tetromino.second.value
                            }
                        } else {
                            if j == tetromino.first.x as usize {
                                &tetromino.first.value
                            } else {
                                &tetromino.second.value
                            }
                        });
                    } else {
                        print!("{}", tetromino.first.value);
                    }

                    j += if tetromino.first.x == tetromino.second.x && tetromino.first.y == tetromino.second.y - 1 {
                        if i == tetromino.first.y as usize {
                            skip_distance_first
                        } else {
                            skip_distance_second
                        }
                    } else if j == tetromino.first.x as usize {
                        skip_distance_first
                    } else {
                        skip_distance_second
                    };

                    found_tetromino = true;
                    break;
                }
            }

            if !found_tetromino {
                print!("{}", screen[i][j]);
                j += 1;
            }
        }
        println!();
    }
}


fn create_tetronimo(tetrominoes_list: &mut Vec<Tetromino>) {
    let random_number: i32 = random_tetronimo();
    let x_position: i32 = random_tetromino_position();
    let mut tetromino_shape: Tetromino = Tetromino::new(
        Line::new(x_position, 0, String::new()),
        Line::new(0, 1, String::new()),
    );

    match random_number {
        1 => {
            tetromino_shape.first.value.push_str("[ ][ ][ ][ ]");
        }
        2 => {
            tetromino_shape.first.value.push_str("[ ][ ][ ]");
            tetromino_shape.second.value.push_str("[ ]");
            tetromino_shape.second.x = x_position + 2;
        }
        3 => {
            tetromino_shape.first.value.push_str("[ ][ ]");
            tetromino_shape.second.value.push_str("[ ][ ]");
            tetromino_shape.second.x = x_position;
        }
        4 => {
            tetromino_shape.first.value.push_str("[ ][ ]");
            tetromino_shape.second.value.push_str("[ ][ ]");
            tetromino_shape.second.x = x_position + 1;
        }
        5 => {
            tetromino_shape.first.value.push_str("[ ][ ][ ]");
            tetromino_shape.second.value.push_str("[ ]");
            tetromino_shape.second.x = x_position + 1;
        }
        _ => panic!("Invalid tetromino shape"),
    }

    tetrominoes_list.push(tetromino_shape);
}

fn update_tetrominoes(tetrominoes_list: &mut Vec<Tetromino>) {
    for tetromino in tetrominoes_list {
        tetromino.move_tetromino(0, 1);
    }
}

fn process_input() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
}

fn random_tetronimo() -> i32 {
    return rand::thread_rng().gen_range(1..=5) as i32;
}

fn random_tetromino_position() -> i32 {
    return rand::thread_rng().gen_range(1..WIDTH - 4) as i32;
}
