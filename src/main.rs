/* Written by Juan Pablo Gutiérrez */

use rand::Rng;
use std::io::Read;

extern crate termion;
use std::io::{stdout, Write};
use std::process::exit;
use termion::raw::IntoRawMode;
use termion::{async_stdin, clear};

use std::thread;
use std::time::Duration;

const WIDTH: usize = 12; // 2 more to account for the borders
const HEIGHT: usize = 40;

/* Tetromino */
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

    // fn count_characters(&mut self) -> usize {
    //     let mut count = 0;
    //     let mut start_index = 0;

    //     while let Some(index) = &self.value[..].find("[ ]") {
    //         count += 1;
    //         start_index += index + "[ ]".len();
    //     }

    //     count
    // }
}

struct Tetromino {
    first: Line,
    second: Line,
    third: Line,
    fourth: Line,
    multi_line: bool,
    rotation: i32,
    shape_type: i32,
    stationary: bool,
}

impl Tetromino {
    fn new(first_line: Line, second_line: Line, multiple_line: bool, shape_type: i32) -> Tetromino {
        Tetromino {
            first: first_line,
            second: second_line,
            multi_line: multiple_line,
            third: Line::new(0, 0, String::new()),
            fourth: Line::new(0, 0, String::new()),
            rotation: 0,
            shape_type,
            stationary: false,
        }
    }

    fn move_tetromino(
        &mut self,
        x_units: i32,
        y_units: i32,
        game_borders: &mut Vec<usize>,
        stdout: &mut termion::raw::RawTerminal<std::io::Stdout>,
    ) {
        let (collides, positions) = self.collides(game_borders);

        if collides {
            self.stationary = true;

            for (x, y) in positions {
                //writeln!(stdout, "{}: {}", x, y).unwrap();
                game_borders[x] = y;
            }

            // Write the game borders
            for x in 0..game_borders.len() {
                writeln!(stdout, "{}: {}", x, game_borders[x]).unwrap();
            }
            return; 
        }

        self.first.x += x_units;
        self.first.y += y_units;

        self.second.x += x_units;
        self.second.y += y_units;

        self.third.x += x_units;
        self.third.y += y_units;

        self.fourth.x += x_units;
        self.fourth.y += y_units;
    }

    fn blank_tetromino(x_position: i32) -> Tetromino {
        Tetromino::new(
            Line::new(x_position, 1, String::new()),
            Line::new(0, 2, String::new()),
            false,
            0,
        )
    }

    /// Checks if the tetromino collides with the game borders
    ///
    /// # Arguments
    ///
    /// * `game_borders` - A vector containing the game borders
    ///
    /// # Returns
    ///
    /// A tuple containing a boolean indicating if the tetromino collides with the game borders and a vector of tuples containing the x and y colliding positions
    fn collides(&mut self, game_borders: &Vec<usize>) -> (bool, Vec<(usize, usize)>) {
        let mut collides: bool = false;
        let mut positions: Vec<(usize, usize)> = Vec::new();

        // Iterates through the game borders
        for x in 1..game_borders.len() - 1 {
            // Checks if the fourth line is between the X coordinate [to reduce processing]
            // And if the fourth line is empty (as it is not always used)

            if self.fourth.x <= x as i32
                && self.fourth.value.len() / 3 + self.fourth.x as usize >= x
                && (!self.fourth.value.is_empty())
            {
                // | Iterates through the number of characters in the line
                for i in 0..self.fourth.value.len() / 3 {
                    // Checks if the x coordinate of the game border is the same as the fourth line  
                    // by adding the index of the character in the line [to check for each length of the character, that way we can check for multiple characters in the line]
                    // and if the y coordinate of the game border is the same as the fourth line
                    if self.fourth.x + i as i32 == x as i32
                        && game_borders[x] == (self.fourth.y + 1) as usize
                    {
                        // If the conditions are met, the tetromino collides with the game borders
                        // and the x and y coordinates are added to the positions vector
                        collides = true;
                        positions.push((x, (self.first.y -1) as usize));
                    }
                }
            }
            // Apply to the rest previous process to the rest of the lines
            if self.third.x <= x as i32
                && self.third.value.len() / 3 + self.third.x as usize >= x
                && (!self.third.value.is_empty())
            {
                for i in 0..self.third.value.len() / 3 {
                    if self.third.x + i as i32 == x as i32
                        && game_borders[x] == (self.third.y + 1) as usize
                    {
                        collides = true;
                        positions.push((x, (self.first.y  -1 )as usize));
                    }
                }
            }
            if self.second.x <= x as i32
                && self.second.value.len() / 3 + self.second.x as usize >= x
                && (!self.second.value.is_empty())
            {
                for i in 0..self.second.value.len() / 3 {
                    if self.second.x + i as i32 == x as i32
                        && game_borders[x] == (self.second.y + 1) as usize
                    {
                        collides = true;
                        positions.push((x,( self.first.y  - 1 ) as usize));
                    }
                }
            }

            // As the first line is always used, skip the check if the line is empty
            // Apply the same process as the previous lines
            if self.first.x <= x as i32 && self.first.value.len() / 3 + self.first.x as usize >= x {
                for i in 0..self.first.value.len() / 3 {
                    if self.first.x + i as i32 == x as i32
                        && game_borders[x] == (self.first.y + 1) as usize
                    {
                        collides = true;
                        positions.push((x, (self.first.y - 1) as usize));
                    }
                }
            }
        }

        return (collides, positions);
    }

    fn rotate(&mut self, rotation: i32) {
        if self.stationary {
            return;
        }

        let mut new_rotation = self.rotation + rotation;
        let normalized_rotation = match new_rotation % 360 {
            r if r < 0 => r + 360,
            r => r,
        } % 360;

        new_rotation = match normalized_rotation {
            360 => 0,
            r => r,
        };

        if self.shape_type == 3 {
            return;
        }

        self.rotation = new_rotation;

        match self.rotation {
            90 => match self.shape_type {
                1 => {
                    self.first.value.clear();
                    self.second.value.clear();
                    self.third.value.clear();
                    self.fourth.value.clear();
                    self.first.value.push_str("[ ]");
                    self.second.value.push_str("[ ]");
                    self.third.value.push_str("[ ]");
                    self.fourth.value.push_str("[ ]");

                    self.second.x = self.first.x;
                    self.second.y = self.first.y + 1;
                    self.third.x = self.first.x;
                    self.third.y = self.first.y + 2;
                    self.fourth.x = self.first.x;
                    self.fourth.y = self.first.y + 3;
                }
                2 => {
                    self.first.value.clear();
                    self.second.value.clear();
                    self.third.value.clear();
                    self.fourth.value.clear();

                    self.first.value.push_str("[ ][ ]");
                    self.second.value.push_str("[ ]");
                    self.third.value.push_str("[ ]");

                    self.second.x = self.first.x;
                    self.second.y = self.first.y + 1;
                    self.third.x = self.first.x;
                    self.third.y = self.first.y + 2;
                }
                4 => {
                    self.first.value.clear();
                    self.second.value.clear();
                    self.third.value.clear();
                    self.fourth.value.clear();

                    self.first.value.push_str("[ ]");
                    self.second.value.push_str("[ ][ ]");
                    self.third.value.push_str("[ ]");

                    self.second.x = self.first.x - 1;
                    self.second.y = self.first.y + 1;
                    self.third.x = self.first.x - 1;
                    self.third.y = self.first.y + 2;
                }
                5 => {
                    self.first.value.clear();
                    self.second.value.clear();
                    self.third.value.clear();
                    self.fourth.value.clear();

                    self.first.value.push_str("[ ]");
                    self.second.value.push_str("[ ][ ]");
                    self.third.value.push_str("[ ]");

                    self.second.x = self.first.x;
                    self.second.y = self.first.y + 1;
                    self.third.x = self.first.x;
                    self.third.y = self.first.y + 2;
                }
                _ => {}
            },
            180 => match self.shape_type {
                1 => {
                    self.first.value.clear();
                    self.second.value.clear();
                    self.third.value.clear();
                    self.fourth.value.clear();
                    self.first.value.push_str("[ ][ ][ ][ ]");
                }
                2 => {
                    self.first.value.clear();
                    self.second.value.clear();
                    self.third.value.clear();
                    self.fourth.value.clear();

                    self.first.value.push_str("[ ]");
                    self.second.value.push_str("[ ][ ][ ]");

                    self.second.x = self.first.x;
                    self.second.y = self.first.y + 1;
                    self.third.x = self.first.x;
                    self.third.y = self.first.y + 2;
                }
                4 => {
                    self.first.value.clear();
                    self.second.value.clear();
                    self.third.value.clear();
                    self.fourth.value.clear();

                    self.first.value.push_str("[ ][ ]");
                    self.second.value.push_str("[ ][ ]");
                    self.second.x = self.first.x + 1;
                    self.multi_line = true;
                    self.shape_type = 4;
                }
                5 => {
                    self.first.value.clear();
                    self.second.value.clear();
                    self.third.value.clear();
                    self.fourth.value.clear();

                    self.first.value.push_str("[ ]");
                    self.second.value.push_str("[ ][ ][ ]");
                    self.second.x = self.first.x - 1;
                    self.multi_line = true;
                    self.shape_type = 5;
                }
                _ => {}
            },
            270 => match self.shape_type {
                1 => {
                    self.first.value.clear();
                    self.second.value.clear();
                    self.third.value.clear();
                    self.fourth.value.clear();
                    self.first.value.push_str("[ ]");
                    self.second.value.push_str("[ ]");
                    self.third.value.push_str("[ ]");
                    self.fourth.value.push_str("[ ]");

                    self.second.x = self.first.x;
                    self.second.y = self.first.y + 1;
                    self.third.x = self.first.x;
                    self.third.y = self.first.y + 2;
                    self.fourth.x = self.first.x;
                    self.fourth.y = self.first.y + 3;
                }
                2 => {
                    self.first.value.clear();
                    self.second.value.clear();
                    self.third.value.clear();
                    self.fourth.value.clear();

                    self.first.value.push_str("[ ]");
                    self.second.value.push_str("[ ]");
                    self.third.value.push_str("[ ][ ]");

                    self.second.x = self.first.x;
                    self.second.y = self.first.y + 1;
                    self.third.x = self.first.x;
                    self.third.y = self.first.y + 2;
                }
                4 => {
                    self.first.value.clear();
                    self.second.value.clear();
                    self.third.value.clear();
                    self.fourth.value.clear();

                    self.first.value.push_str("[ ]");
                    self.second.value.push_str("[ ][ ]");
                    self.third.value.push_str("[ ]");

                    self.second.x = self.first.x - 1;
                    self.second.y = self.first.y + 1;
                    self.third.x = self.first.x - 1;
                    self.third.y = self.first.y + 2;
                }
                5 => {
                    self.first.value.clear();
                    self.second.value.clear();
                    self.third.value.clear();
                    self.fourth.value.clear();

                    self.first.value.push_str("[ ]");
                    self.second.value.push_str("[ ][ ]");
                    self.third.value.push_str("[ ]");

                    self.second.x = self.first.x - 1;
                    self.second.y = self.first.y + 1;
                    self.third.x = self.first.x;
                    self.third.y = self.first.y + 2;
                }
                _ => {}
            },
            0 => match self.shape_type {
                1 => {
                    self.first.value.clear();
                    self.second.value.clear();
                    self.third.value.clear();
                    self.fourth.value.clear();

                    self.first.value.push_str("[ ][ ][ ][ ]");
                }
                2 => {
                    self.first.value.clear();
                    self.second.value.clear();
                    self.third.value.clear();
                    self.fourth.value.clear();

                    self.first.value.push_str("[ ][ ][ ]");
                    self.second.value.push_str("[ ]");
                    self.second.x = self.first.x + 2;
                    self.multi_line = true;
                    self.shape_type = 2;
                }
                4 => {
                    self.first.value.clear();
                    self.second.value.clear();
                    self.third.value.clear();
                    self.fourth.value.clear();

                    self.first.value.push_str("[ ][ ]");
                    self.second.value.push_str("[ ][ ]");
                    self.second.x = self.first.x + 1;
                    self.multi_line = true;
                    self.shape_type = 4;
                }
                5 => {
                    self.first.value.clear();
                    self.second.value.clear();
                    self.third.value.clear();
                    self.fourth.value.clear();

                    self.first.value.push_str("[ ][ ][ ]");
                    self.second.value.push_str("[ ]");
                    self.second.x = self.first.x + 1;
                    self.multi_line = true;
                    self.shape_type = 5;
                }
                _ => {}
            },
            _ => {}
        }
    }
}

/* Game loop */

fn main() {
    let mut screen: [[&str; WIDTH]; HEIGHT] = [[""; WIDTH]; HEIGHT];
    let mut rendered_tetrominoes_list: Vec<Tetromino> = Vec::new();
    let mut unrendered_tetrominoes_list: Vec<Tetromino> = Vec::new();

    let game_borders: [usize; WIDTH] = [HEIGHT; WIDTH];

    create_screen(&mut screen);
    create_tetronimo(&mut unrendered_tetrominoes_list);

    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    let mut creation_counter = 1;
    let mut movement_counter = 1;

    //writeln!(stdout, "{}{}", clear::All, termion::cursor::Hide).unwrap();
    display_screen(
        &screen,
        &mut unrendered_tetrominoes_list,
        &mut rendered_tetrominoes_list,
        &mut stdout,
    );

    loop {
        //write!(stdout, "{}", termion::clear::CurrentLine).unwrap();

        let b = stdin.next();

        if let Some(Ok(b'q')) = b {
            break;
        }
        if let Some(Ok(b'a')) = b {
            move_tetrmonioes(
                &mut unrendered_tetrominoes_list,
                -1,
                &mut game_borders.to_vec(),
                &mut stdout,
            );
            display_screen(
                &screen,
                &mut unrendered_tetrominoes_list,
                &mut rendered_tetrominoes_list,
                &mut stdout,
            );
        }
        if let Some(Ok(b'd')) = b {
            move_tetrmonioes(
                &mut unrendered_tetrominoes_list,
                1,
                &mut game_borders.to_vec(),
                &mut stdout,
            );
            display_screen(
                &screen,
                &mut unrendered_tetrominoes_list,
                &mut rendered_tetrominoes_list,
                &mut stdout,
            );
        }
        if let Some(Ok(b'r')) = b {
            rotate_tetrominoes(&mut unrendered_tetrominoes_list, 90);
            display_screen(
                &screen,
                &mut unrendered_tetrominoes_list,
                &mut rendered_tetrominoes_list,
                &mut stdout,
            );
        }

        thread::sleep(Duration::from_millis(100));

        if creation_counter % 70 == 0 {
            create_tetronimo(&mut unrendered_tetrominoes_list);
        }
        creation_counter += 1;

        if movement_counter % 5 == 0 {
            update_tetrominoes(
                &mut unrendered_tetrominoes_list,
                &mut game_borders.to_vec(),
                &mut stdout,
            );
        }
        movement_counter += 1;

        display_screen(
            &screen,
            &mut unrendered_tetrominoes_list,
            &mut rendered_tetrominoes_list,
            &mut stdout,
        );

        stdout.flush().unwrap();
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
    stdout: &mut termion::raw::RawTerminal<std::io::Stdout>,
) {
    //writeln!(stdout, "{}{}", clear::All, termion::cursor::Hide).unwrap();

    for i in 0..HEIGHT {
        let mut j = 0;
        while j < WIDTH {
            let mut found_tetromino = false;
            let mut x = 0;

            while !unrendered_tetrominoes.is_empty() && x < unrendered_tetrominoes.len() {
                let tetromino = &unrendered_tetrominoes[x];

                let skip_distance_first = (tetromino.first.value.len() / 3) as usize;
                let skip_distance_second = (tetromino.second.value.len() / 3) as usize;

                let skip_distance_third = (tetromino.third.value.len() / 3) as usize;

                let skip_distance_fourth = (tetromino.fourth.value.len() / 3) as usize;

                if tetromino.rotation == 0 || tetromino.shape_type == 3 {
                    if tetromino.first.x as usize == j && tetromino.first.y as usize == i {
                        write!(stdout, "{}", tetromino.first.value).unwrap();
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
                        write!(stdout, "{}", tetromino.second.value).unwrap();
                        rendered_tetrominoes.push(unrendered_tetrominoes.remove(x));
                        break;
                    }
                } else {
                    if tetromino.first.x as usize == j && tetromino.first.y as usize == i {
                        if (tetromino.shape_type == 1 && tetromino.rotation == 180) {
                            write!(stdout, "{}", tetromino.first.value).unwrap();
                            stdout.flush().unwrap();

                            j += skip_distance_first;
                            found_tetromino = true;
                            if tetromino.multi_line == false {
                                rendered_tetrominoes.push(unrendered_tetrominoes.remove(x));
                            }
                            break;
                        } else {
                            write!(stdout, "{}", tetromino.first.value).unwrap();
                            stdout.flush().unwrap();
                            j += skip_distance_first;
                            found_tetromino = true;
                            break;
                        }
                    } else if tetromino.second.x as usize == j && tetromino.second.y as usize == i {
                        found_tetromino = true;
                        j += skip_distance_second;
                        write!(stdout, "{}", tetromino.second.value).unwrap();
                        stdout.flush().unwrap();
                        break;
                    } else if tetromino.third.x as usize == j && tetromino.third.y as usize == i {
                        if tetromino.third.value.is_empty() {
                            break;
                        } else {
                            found_tetromino = true;
                            j += skip_distance_third;
                            write!(stdout, "{}", tetromino.third.value).unwrap();
                            stdout.flush().unwrap();
                            if tetromino.shape_type != 1 {
                                rendered_tetrominoes.push(unrendered_tetrominoes.remove(x));
                            }
                            break;
                        }
                    } else if tetromino.shape_type == 1
                        && tetromino.fourth.x as usize == j
                        && tetromino.fourth.y as usize == i
                    {
                        found_tetromino = true;
                        j += skip_distance_fourth;
                        write!(stdout, "{}", tetromino.fourth.value).unwrap();
                        stdout.flush().unwrap();
                        rendered_tetrominoes.push(unrendered_tetrominoes.remove(x));
                        break;
                    }
                }

                x += 1;
            }

            if !found_tetromino {
                write!(stdout, "{}", screen[i][j]).unwrap();
                j += 1;
            }
        }
        write!(stdout, "\n\r").unwrap();
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
            tetromino_shape.shape_type = 1;
        }
        2 => {
            tetromino_shape.first.value.push_str("[ ][ ][ ]");
            tetromino_shape.second.value.push_str("[ ]");
            tetromino_shape.second.x = x_position + 2;
            tetromino_shape.multi_line = true;
            tetromino_shape.shape_type = 2;
        }
        3 => {
            tetromino_shape.first.value.push_str("[ ][ ]");
            tetromino_shape.second.value.push_str("[ ][ ]");
            tetromino_shape.second.x = x_position;
            tetromino_shape.multi_line = true;
            tetromino_shape.shape_type = 3;
        }
        4 => {
            tetromino_shape.first.value.push_str("[ ][ ]");
            tetromino_shape.second.value.push_str("[ ][ ]");
            tetromino_shape.second.x = x_position + 1;
            tetromino_shape.multi_line = true;
            tetromino_shape.shape_type = 4;
        }
        5 => {
            tetromino_shape.first.value.push_str("[ ][ ][ ]");
            tetromino_shape.second.value.push_str("[ ]");
            tetromino_shape.second.x = x_position + 1;
            tetromino_shape.multi_line = true;
            tetromino_shape.shape_type = 5;
        }
        _ => panic!("Invalid tetromino shape"),
    }

    tetrominoes_list.push(tetromino_shape);
}

fn move_tetrmonioes(
    tetrominoes_list: &mut Vec<Tetromino>,
    movement: i32,
    game_borders: &mut Vec<usize>,
    stdout: &mut termion::raw::RawTerminal<std::io::Stdout>,
) {
    for tetromino in tetrominoes_list {
        tetromino.move_tetromino(movement, 0, game_borders, stdout);
    }
}

fn rotate_tetrominoes(tetrominoes_list: &mut Vec<Tetromino>, rotation: i32) {
    for tetromino in tetrominoes_list {
        tetromino.rotate(rotation);
        print!("{}", tetromino.rotation.to_string());
    }
}

fn update_tetrominoes(
    tetrominoes_list: &mut Vec<Tetromino>,
    game_borders: &mut Vec<usize>,
    stdout: &mut termion::raw::RawTerminal<std::io::Stdout>,
) {
    for tetromino in tetrominoes_list {
        tetromino.move_tetromino(0, 1, game_borders, stdout);
    }
}

fn random_tetronimo() -> i32 {
    return rand::thread_rng().gen_range(1..=5) as i32;
}

fn random_tetromino_position() -> i32 {
    return rand::thread_rng().gen_range(2..WIDTH - 4) as i32;
}
