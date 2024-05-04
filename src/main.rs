/* Written by Juan Pablo Gutiérrez */

use rand::Rng;
use std::io;

const WIDTH: usize = 12; // 2 more to account for
const HEIGHT: usize = 40;

struct Line {
    x: i32,
    y: i32,
    value: String,
}

impl Line {
    fn new(x_pos: i32, y_pos: i32, value_string: String) -> Line {
        Line {
            x: x_pos,
            y: y_pos,
            value: value_string,
        }
    }
}

struct Tetromino {
    first: Line,
    second: Line,
    multi_line: bool,
}

impl Tetromino {
    fn new(first_line: Line, second_line: Line, multiple_line: bool) -> Tetromino {
        Tetromino {
            first: first_line,
            second: second_line,
            multi_line: multiple_line,
        }
    }

    fn move_tetromino(&mut self, x_units: i32, y_units: i32) {
        self.first.x += x_units;
        self.first.y += y_units;

        self.second.x += x_units;
        self.second.y += y_units;
    }

    fn blank_tetromino(x_position: i32) -> Tetromino {
        Tetromino::new(
            Line::new(x_position, 0, String::new()),
            Line::new(0, 1, String::new()),
            false,
        )
    }
}

fn main() {
    let mut screen: [[&str; WIDTH]; HEIGHT] = [[""; WIDTH]; HEIGHT];
    let mut rendered_tetrominoes_list: Vec<Tetromino> = Vec::new();
    let mut unrendered_tetrominoes_list: Vec<Tetromino> = Vec::new();

    create_screen(&mut screen);
    create_tetronimo(&mut unrendered_tetrominoes_list);
    display_screen(
        &screen,
        &mut unrendered_tetrominoes_list,
        &mut rendered_tetrominoes_list,
    );

    let mut counter = 1;

    loop {
        process_input();
        update_tetrominoes(&mut unrendered_tetrominoes_list);
        display_screen(
            &screen,
            &mut unrendered_tetrominoes_list,
            &mut rendered_tetrominoes_list,
        );
        println!("Before: {}", unrendered_tetrominoes_list.len());
        if counter % 5 == 0 {
            create_tetronimo(&mut unrendered_tetrominoes_list);
        }
        println!("After: {}", unrendered_tetrominoes_list.len());
        counter += 1;

        if counter > 20 {
            break;
        }
    }
}

fn create_screen(screen: &mut [[&str; WIDTH]; HEIGHT]) {
    for i in 0..HEIGHT {
        screen[i][0] = "<!";
        for j in 1..WIDTH - 1 {
            screen[i][j] = " . ";
        }
        screen[i][WIDTH - 1] = "!>";
    }
}

fn display_screen(
    screen: &[[&str; WIDTH]; HEIGHT],
    unrendered_tetrominoes: &mut Vec<Tetromino>,
    rendered_tetrominoes: &mut Vec<Tetromino>,
) {
    println!("Unrendered: {}", unrendered_tetrominoes.len());
    println!("Rendered: {}", rendered_tetrominoes.len());
    for i in 0..HEIGHT {
        let mut j = 0;
        while j < WIDTH {
            let mut found_tetromino = false;
            let mut x = 0;

            while !unrendered_tetrominoes.is_empty() && x < unrendered_tetrominoes.len() {
                let tetromino = &unrendered_tetrominoes[x];

                let skip_distance_first = (tetromino.first.value.len() / 3) as usize;
                let skip_distance_second = if tetromino.multi_line == false {
                    0
                } else {
                    (tetromino.second.value.len() / 3) as usize
                };

                if tetromino.first.x as usize == j && tetromino.first.y as usize == i {
                    print!("{}", tetromino.first.value);
                    j += skip_distance_first;
                    found_tetromino = true;
                    if tetromino.multi_line == false {
                        rendered_tetrominoes.push(unrendered_tetrominoes.remove(x));
                    }
                    break;
                } else if tetromino.multi_line == true
                    && (tetromino.second.x as usize == j && tetromino.second.y as usize == i)
                {
                    found_tetromino = true;
                    j += skip_distance_second;
                    print!("{}", tetromino.second.value);
                    rendered_tetrominoes.push(unrendered_tetrominoes.remove(x));
                    break;
                }

                x += 1;
            }

            if !found_tetromino {
                print!("{}", screen[i][j]);
                j += 1;
            }
        }
        println!();
    }

    unrendered_tetrominoes.append(rendered_tetrominoes);
}

fn create_tetronimo(tetrominoes_list: &mut Vec<Tetromino>) {
    let random_number: i32 = random_tetronimo();
    let x_position: i32 = random_tetromino_position();
    let mut tetromino_shape: Tetromino = Tetromino::blank_tetromino(x_position);

    match random_number {
        1 => {
            tetromino_shape.first.value.push_str("[ ][ ][ ][ ]");
        }
        2 => {
            tetromino_shape.first.value.push_str("[ ][ ][ ]");
            tetromino_shape.second.value.push_str("[ ]");
            tetromino_shape.second.x = x_position + 2;
            tetromino_shape.multi_line = true;
        }
        3 => {
            tetromino_shape.first.value.push_str("[ ][ ]");
            tetromino_shape.second.value.push_str("[ ][ ]");
            tetromino_shape.second.x = x_position;
            tetromino_shape.multi_line = true;
        }
        4 => {
            tetromino_shape.first.value.push_str("[ ][ ]");
            tetromino_shape.second.value.push_str("[ ][ ]");
            tetromino_shape.second.x = x_position + 1;
            tetromino_shape.multi_line = true;
        }
        5 => {
            tetromino_shape.first.value.push_str("[ ][ ][ ]");
            tetromino_shape.second.value.push_str("[ ]");
            tetromino_shape.second.x = x_position + 1;
            tetromino_shape.multi_line = true;
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
    let mut input: String = String::new();
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
