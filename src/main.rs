/* Written by Juan Pablo Gutiérrez */

use rand::Rng;
use std::io::Read;

extern crate termion;
use std::io::{stdout, Write};
use termion::raw::IntoRawMode;
use termion::{async_stdin, clear};

use std::fmt;
use std::thread;
use std::time::Duration;

const WIDTH: usize = 12; // 2 more to account for the borders
const HEIGHT: usize = 40;

/* Tetromino */
struct TetrominoCharacter {
    x: i32,
    y: i32,
    value: &'static str, // Use &'static str for string literals
}

struct Line {
    x: i32,                              //Represents thes start of the line
    y: i32,                              // Represents the y position of the line
    characters: Vec<TetrominoCharacter>, // List of TetrominoCharacter
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for character in &self.characters {
            write!(f, "{}", character.value)?;
        }
        Ok(())
    }
}

impl Line {
    fn new(x_pos: i32, y_pos: i32, num: i32) -> Line {
        Line {
            x: x_pos,
            y: y_pos,
            characters: Line::create_characters(x_pos, y_pos, num),
        }
    }

    fn create_characters(x_pos: i32, y_pos: i32, num: i32) -> Vec<TetrominoCharacter> {
        let mut characters: Vec<TetrominoCharacter> = Vec::new();

        for i in 0..num {
            characters.push(TetrominoCharacter {
                x: x_pos + i,
                y: y_pos,
                value: "[ ]",
            });
        }

        characters
    }

    fn move_line(&mut self, x_units: i32, y_units: i32) {
        self.x += x_units;
        self.y += y_units;

        for character in &mut self.characters {
            character.x += x_units;
            character.y += y_units;
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
            third: Line::new(0, 0, 0),
            fourth: Line::new(0, 0, 0),
            rotation: 0,
            shape_type,
            stationary: false,
        }
    }

    fn move_tetromino(&mut self, x_units: i32, y_units: i32, game_borders: &mut Vec<usize>) {
        let collides: bool = self.collides(&game_borders);

        if self.stationary {
            return;
        }

        if collides {
            self.stationary = true;

            self.remake_gameborders(game_borders);
        } else {
            self.first.move_line(x_units, y_units);
            self.second.move_line(x_units, y_units);
            self.third.move_line(x_units, y_units);
            self.fourth.move_line(x_units, y_units);
        }
    }

    fn blank_tetromino(x_position: i32) -> Tetromino {
        Tetromino::new(Line::new(x_position, 1, 0), Line::new(0, 2, 0), false, 0)
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
    fn collides(&mut self, game_borders: &Vec<usize>) -> bool {
        let mut collides: bool = false;

        // Iterates through the game borders
        for x in 1..game_borders.len() - 1 {
            // Checks if the fourth line is between the X coordinate [to reduce processing]
            // And if the fourth line is empty (as it is not always used)

            for character in &self.fourth.characters {
                if character.x == x as i32 && game_borders[x] == (self.fourth.y + 1) as usize {
                    collides = true;
                }
            }

            for character in &self.third.characters {
                if character.x == x as i32 && game_borders[x] == (self.third.y + 1) as usize {
                    collides = true;
                }
            }

            for character in &self.second.characters {
                if character.x == x as i32 && game_borders[x] == (self.second.y + 1) as usize {
                    collides = true;
                }
            }

            for character in &self.first.characters {
                if character.x == x as i32 && game_borders[x] == (self.first.y + 1) as usize {
                    collides = true;
                }
            }
        }

        return collides;
    }

    fn remake_gameborders(&mut self, game_borders: &mut Vec<usize>) {
        if !self.fourth.to_string().is_empty() {
            for fourth_character in &self.fourth.characters {
                game_borders[fourth_character.x as usize] = (fourth_character.y) as usize;
            }
        }

        if !self.third.to_string().is_empty() {
            for third_character in &self.third.characters {
                game_borders[third_character.x as usize] = (third_character.y) as usize;
            }
        }

        if !self.second.to_string().is_empty() {
            for second_character in &self.second.characters {
                game_borders[second_character.x as usize] = (second_character.y) as usize;
            }
        }

        for first_character in &self.first.characters {
            game_borders[first_character.x as usize] = (first_character.y) as usize;
        }
    }

    fn clear(&mut self) {
        self.first.characters.clear();
        self.second.characters.clear();
        self.third.characters.clear();
        self.fourth.characters.clear();
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
                    self.clear();

                    self.first = Line::new(self.first.x, self.first.y, 1);
                    self.second = Line::new(self.first.x, self.first.y + 1, 1);
                    self.third = Line::new(self.first.x, self.first.y + 2, 1);
                    self.fourth = Line::new(self.first.x, self.first.y + 3, 1);
                }
                2 => {
                    self.clear();

                    self.first = Line::new(self.first.x, self.first.y, 2);
                    self.second = Line::new(self.first.x, self.first.y + 1, 1);
                    self.third = Line::new(self.first.x, self.first.y + 2, 1);
                }
                4 => {
                    self.clear();

                    self.first = Line::new(self.first.x, self.first.y, 1);
                    self.second = Line::new(self.first.x - 1, self.first.y + 1, 2);
                    self.third = Line::new(self.first.x - 1, self.first.y + 2, 1);
                }
                5 => {
                    self.clear();

                    self.first = Line::new(self.first.x, self.first.y, 1);
                    self.second = Line::new(self.first.x, self.first.y + 1, 2);
                    self.third = Line::new(self.first.x, self.first.y + 2, 1);
                }
                _ => {}
            },
            180 => match self.shape_type {
                1 => {
                    self.clear();

                    self.first = Line::new(self.first.x, self.first.y, 4);
                }
                2 => {
                    self.clear();

                    self.first = Line::new(self.first.x, self.first.y, 1);
                    self.second = Line::new(self.first.x, self.first.y + 1, 3);
                }
                4 => {
                    self.clear();

                    self.first = Line::new(self.first.x, self.first.y, 2);
                    self.second = Line::new(self.first.x + 1, self.first.y + 1, 2);
                }
                5 => {
                    self.clear();

                    self.first = Line::new(self.first.x, self.first.y, 1);
                    self.second = Line::new(self.first.x - 1, self.first.y + 1, 3);
                }
                _ => {}
            },
            270 => match self.shape_type {
                1 => {
                    self.clear();

                    self.first = Line::new(self.first.x, self.first.y, 1);
                    self.second = Line::new(self.first.x, self.first.y + 1, 1);
                    self.third = Line::new(self.first.x, self.first.y + 2, 1);
                    self.fourth = Line::new(self.first.x, self.first.y + 3, 1);
                }
                2 => {
                    self.clear();

                    self.first = Line::new(self.first.x, self.first.y, 1);
                    self.second = Line::new(self.first.x, self.first.y + 1, 1);
                    self.third = Line::new(self.first.x, self.first.y + 2, 2);
                }
                4 => {
                    self.clear();

                    self.first = Line::new(self.first.x, self.first.y, 1);
                    self.second = Line::new(self.first.x - 1, self.first.y + 1, 2);
                    self.third = Line::new(self.first.x - 1, self.first.y + 2, 1);
                }
                5 => {
                    self.clear();

                    self.first = Line::new(self.first.x, self.first.y, 1);
                    self.second = Line::new(self.first.x - 1, self.first.y + 1, 2);
                    self.third = Line::new(self.first.x, self.first.y + 2, 1);

                    self.second.x = self.first.x - 1;
                    self.second.y = self.first.y + 1;
                    self.third.x = self.first.x;
                    self.third.y = self.first.y + 2;
                }
                _ => {}
            },
            0 => match self.shape_type {
                1 => {
                    self.clear();

                    self.first = Line::new(self.first.x, self.first.y, 4);
                }
                2 => {
                    self.clear();

                    self.first = Line::new(self.first.x, self.first.y, 3);
                    self.second = Line::new(self.first.x + 2, self.first.y + 1, 1);
                }
                4 => {
                    self.clear();

                    self.first = Line::new(self.first.x, self.first.y, 2);
                    self.second = Line::new(self.first.x + 1, self.first.y + 1, 2);
                }
                5 => {
                    self.clear();

                    self.first = Line::new(self.first.x, self.first.y, 3);
                    self.second = Line::new(self.first.x + 1, self.first.y + 1, 1);
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

    let mut game_borders: [usize; WIDTH] = [HEIGHT; WIDTH];

    create_screen(&mut screen);
    create_tetronimo(&mut unrendered_tetrominoes_list);

    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    let mut creation_counter = 1;
    let mut movement_counter = 1;

    writeln!(stdout, "{}{}", clear::All, termion::cursor::Hide).unwrap();
    display_screen(
        &screen,
        &mut unrendered_tetrominoes_list,
        &mut rendered_tetrominoes_list,
        &mut stdout,
        &game_borders,
    );

    loop {
        write!(stdout, "{}", termion::clear::CurrentLine).unwrap();

        let b = stdin.next();

        if let Some(Ok(b'q')) = b {
            break;
        }
        if let Some(Ok(b'a')) = b {
            let vec: Vec<usize> = move_tetrmonioes(
                &mut unrendered_tetrominoes_list,
                -1,
                game_borders.to_vec(),
            );

            game_borders.copy_from_slice(&vec);

            display_screen(
                &screen,
                &mut unrendered_tetrominoes_list,
                &mut rendered_tetrominoes_list,
                &mut stdout,
                &game_borders,
            );
        }
        if let Some(Ok(b'd')) = b {
            let vec: Vec<usize> = move_tetrmonioes(
                &mut unrendered_tetrominoes_list,
                1,
                game_borders.to_vec(),
            );
            display_screen(
                &screen,
                &mut unrendered_tetrominoes_list,
                &mut rendered_tetrominoes_list,
                &mut stdout,
                &game_borders,
            );

            game_borders.copy_from_slice(&vec);
        }
        if let Some(Ok(b'r')) = b {
            rotate_tetrominoes(&mut unrendered_tetrominoes_list, 90);
            display_screen(
                &screen,
                &mut unrendered_tetrominoes_list,
                &mut rendered_tetrominoes_list,
                &mut stdout,
                &game_borders,
            );
        }

        thread::sleep(Duration::from_millis(100));

        let mut create : bool = true;

        for tetromino in &unrendered_tetrominoes_list {
            if !tetromino.stationary {
                create = false;
                break;
            }
        }

        if create {
            create_tetronimo(&mut unrendered_tetrominoes_list);
        }

        creation_counter += 1;

        if movement_counter % 2 == 0 {
            let vec: Vec<usize> = update_tetrominoes(
                &mut unrendered_tetrominoes_list,
                game_borders.to_vec(),
            );

            game_borders.copy_from_slice(&vec);
        }
        movement_counter += 1;

        display_screen(
            &screen,
            &mut unrendered_tetrominoes_list,
            &mut rendered_tetrominoes_list,
            &mut stdout,
            &game_borders,
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
    game_borders: &[usize; WIDTH],
) {
    writeln!(stdout, "{}{}", clear::All, termion::cursor::Hide).unwrap();

    for i in 0..HEIGHT {
        let mut j = 0;
        while j < WIDTH {
            let mut found_tetromino = false;
            let mut x = 0;

            while !unrendered_tetrominoes.is_empty() && x < unrendered_tetrominoes.len() {
                let tetromino = &unrendered_tetrominoes[x];

                let skip_distance_first = (tetromino.first.characters.len()) as usize;
                let skip_distance_second = (tetromino.second.characters.len()) as usize;

                let skip_distance_third = (tetromino.third.characters.len()) as usize;

                let skip_distance_fourth = (tetromino.fourth.characters.len()) as usize;

                if tetromino.rotation == 0 || tetromino.shape_type == 3 {
                    if tetromino.first.x as usize == j && tetromino.first.y as usize == i {
                        write!(stdout, "{}", tetromino.first).unwrap();
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
                        write!(stdout, "{}", tetromino.second).unwrap();
                        rendered_tetrominoes.push(unrendered_tetrominoes.remove(x));
                        break;
                    }
                } else {
                    if tetromino.first.x as usize == j && tetromino.first.y as usize == i {
                        if tetromino.shape_type == 1 && tetromino.rotation == 180 {
                            write!(stdout, "{}", tetromino.first).unwrap();
                            stdout.flush().unwrap();

                            j += skip_distance_first;
                            found_tetromino = true;
                            if tetromino.multi_line == false {
                                rendered_tetrominoes.push(unrendered_tetrominoes.remove(x));
                            }
                            break;
                        } else {
                            write!(stdout, "{}", tetromino.first).unwrap();
                            stdout.flush().unwrap();
                            j += skip_distance_first;
                            found_tetromino = true;
                            break;
                        }
                    } else if tetromino.second.x as usize == j && tetromino.second.y as usize == i {
                        found_tetromino = true;
                        j += skip_distance_second;
                        write!(stdout, "{}", tetromino.second).unwrap();
                        stdout.flush().unwrap();
                        break;
                    } else if tetromino.third.x as usize == j && tetromino.third.y as usize == i {
                        if tetromino.third.to_string().is_empty() {
                            break;
                        } else {
                            found_tetromino = true;
                            j += skip_distance_third;
                            write!(stdout, "{}", tetromino.third).unwrap();
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
                        write!(stdout, "{}", tetromino.fourth).unwrap();
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

    // Write the game borders
    // for x in 0..game_borders.len() {
    //     write!(stdout, "{} ", game_borders[x]).unwrap();
    // }

    // write!(stdout, "\n\r").unwrap();

    unrendered_tetrominoes.append(rendered_tetrominoes);
}

fn create_tetronimo(tetrominoes_list: &mut Vec<Tetromino>) {
    let random_number: i32 = random_tetronimo();
    let x_position: i32 = random_tetromino_position();
    let mut tetromino_shape: Tetromino = Tetromino::blank_tetromino(x_position);

    match random_number {
        1 => {
            tetromino_shape.first.characters =
                Line::create_characters(x_position, tetromino_shape.first.y, 4);
            tetromino_shape.shape_type = 1;
        }
        2 => {
            tetromino_shape.first.characters =
                Line::create_characters(x_position, tetromino_shape.first.y, 3);
            tetromino_shape.second.characters =
                Line::create_characters(x_position + 2, tetromino_shape.first.y + 1, 1);
            tetromino_shape.second.x = x_position + 2;
            tetromino_shape.multi_line = true;
            tetromino_shape.shape_type = 2;
        }
        3 => {
            tetromino_shape.first.characters =
                Line::create_characters(x_position, tetromino_shape.first.y, 2);
            tetromino_shape.second.characters =
                Line::create_characters(x_position, tetromino_shape.first.y + 1, 2);
            tetromino_shape.second.x = x_position;
            tetromino_shape.multi_line = true;
            tetromino_shape.shape_type = 3;
        }
        4 => {
            tetromino_shape.first.characters =
                Line::create_characters(x_position, tetromino_shape.first.y, 2);
            tetromino_shape.second.characters =
                Line::create_characters(x_position + 1, tetromino_shape.first.y + 1, 2);
            tetromino_shape.second.x = x_position + 1;
            tetromino_shape.multi_line = true;
            tetromino_shape.shape_type = 4;
        }
        5 => {
            tetromino_shape.first.characters =
                Line::create_characters(x_position, tetromino_shape.first.y, 3);
            tetromino_shape.second.characters =
                Line::create_characters(x_position + 1, tetromino_shape.first.y + 1, 1);
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
    mut game_borders: Vec<usize>,
) -> Vec<usize> {
    for tetromino in tetrominoes_list {
        tetromino.move_tetromino(movement, 0, &mut game_borders);
    }

    return game_borders;
}

fn rotate_tetrominoes(tetrominoes_list: &mut Vec<Tetromino>, rotation: i32) {
    for tetromino in tetrominoes_list {
        tetromino.rotate(rotation);
        print!("{}", tetromino.rotation.to_string());
    }
}

fn update_tetrominoes(
    tetrominoes_list: &mut Vec<Tetromino>,
    mut game_borders: Vec<usize>,
) -> Vec<usize> {
    for tetromino in tetrominoes_list {
        tetromino.move_tetromino(0, 1, &mut game_borders);
    }

    return game_borders;
}

/* Random helper methods */

fn random_tetronimo() -> i32 {
    return rand::thread_rng().gen_range(1..=5) as i32;
}

fn random_tetromino_position() -> i32 {
    return rand::thread_rng().gen_range(2..WIDTH - 4) as i32;
}
