/* Written by Juan Pablo Gutiérrez */

use rand::Rng;
use std::io::Read;

extern crate termion;
use std::io::{stdout, Write};
use termion::raw::IntoRawMode;
use termion::{async_stdin, clear};

use std::thread;
use std::time::Duration;

const WIDTH: usize = 12; // 2 more to account for the borders
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
    third: Line,
    fourth: Line,
    multi_line: bool,
    rotation: i32,
    shape: i32,
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
            shape: shape_type,
        }
    }

    fn move_tetromino(&mut self, x_units: i32, y_units: i32) {
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

    fn rotate(&mut self, rotation: i32) {
        let mut new_rotation = self.rotation + rotation;
        let normalized_rotation = match new_rotation % 360 {
            r if r < 0 => r + 360,
            r => r,
        } % 360;

        new_rotation = match normalized_rotation {
            360 => 0,
            r => r,
        };

        if self.shape == 3 {
            return;
        }

        self.rotation = new_rotation;

        match self.rotation {
            90 => match self.shape {
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
            180 => match self.shape {
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
                    self.shape = 4;
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
                    self.shape = 5;
                }
                _ => {}
            },
            270 => match self.shape {
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
            0 => match self.shape {
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
                    self.shape = 2;
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
                    self.shape = 4;
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
                    self.shape = 5;
                }
                _ => {}
            },
            _ => {}
        }
    }
}

fn main() {
    let mut screen: [[&str; WIDTH]; HEIGHT] = [[""; WIDTH]; HEIGHT];
    let mut rendered_tetrominoes_list: Vec<Tetromino> = Vec::new();
    let mut unrendered_tetrominoes_list: Vec<Tetromino> = Vec::new();

    create_screen(&mut screen);
    create_tetronimo(&mut unrendered_tetrominoes_list);

    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    let mut creation_counter = 1;
    let mut movement_counter = 1;

    writeln!(stdout, "{}{}", clear::All, termion::cursor::Hide).unwrap();

    rotate_tetrominoes(&mut unrendered_tetrominoes_list, 90);

    display_screen(
        &screen,
        &mut unrendered_tetrominoes_list,
        &mut rendered_tetrominoes_list,
        &mut stdout,
    );

    loop {
        write!(stdout, "{}", termion::clear::CurrentLine).unwrap();

        let b = stdin.next();

        if let Some(Ok(b'q')) = b {
            break;
        }
        if let Some(Ok(b'a')) = b {
            move_tetrmonioes(&mut unrendered_tetrominoes_list, -1);
            display_screen(
                &screen,
                &mut unrendered_tetrominoes_list,
                &mut rendered_tetrominoes_list,
                &mut stdout,
            );
        }
        if let Some(Ok(b'd')) = b {
            move_tetrmonioes(&mut unrendered_tetrominoes_list, 1);
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
            //create_tetronimo(&mut unrendered_tetrominoes_list);
        }
        creation_counter += 1;

        if movement_counter % 5 == 0 {
            update_tetrominoes(&mut unrendered_tetrominoes_list);
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
    writeln!(stdout, "{}{}", clear::All, termion::cursor::Hide).unwrap();

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

                if tetromino.rotation == 0 || tetromino.shape == 3 {
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
                        if (tetromino.shape == 1 && tetromino.rotation == 180) {
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
                            if tetromino.shape != 1 {
                                rendered_tetrominoes.push(unrendered_tetrominoes.remove(x));
                            }
                            break;
                        }
                    } else if tetromino.shape == 1
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
    let x_position: i32 =  random_tetromino_position();
    let mut tetromino_shape: Tetromino = Tetromino::blank_tetromino(x_position);

    match random_number {
        1 => {
            tetromino_shape.first.value.push_str("[ ][ ][ ][ ]");
            tetromino_shape.shape = 1;
        }
        2 => {
            tetromino_shape.first.value.push_str("[ ][ ][ ]");
            tetromino_shape.second.value.push_str("[ ]");
            tetromino_shape.second.x = x_position + 2;
            tetromino_shape.multi_line = true;
            tetromino_shape.shape = 2;
        }
        3 => {
            tetromino_shape.first.value.push_str("[ ][ ]");
            tetromino_shape.second.value.push_str("[ ][ ]");
            tetromino_shape.second.x = x_position;
            tetromino_shape.multi_line = true;
            tetromino_shape.shape = 3;
        }
        4 => {
            tetromino_shape.first.value.push_str("[ ][ ]");
            tetromino_shape.second.value.push_str("[ ][ ]");
            tetromino_shape.second.x = x_position + 1;
            tetromino_shape.multi_line = true;
            tetromino_shape.shape = 4;
        }
        5 => {
            tetromino_shape.first.value.push_str("[ ][ ][ ]");
            tetromino_shape.second.value.push_str("[ ]");
            tetromino_shape.second.x = x_position + 1;
            tetromino_shape.multi_line = true;
            tetromino_shape.shape = 5;
        }
        _ => panic!("Invalid tetromino shape"),
    }

    tetrominoes_list.push(tetromino_shape);
}

fn move_tetrmonioes(tetrominoes_list: &mut Vec<Tetromino>, movement: i32) {
    for tetromino in tetrominoes_list {
        tetromino.move_tetromino(movement, 0);
    }
}

fn rotate_tetrominoes(tetrominoes_list: &mut Vec<Tetromino>, rotation: i32) {
    for tetromino in tetrominoes_list {
        tetromino.rotate(rotation);
        print!("{}", tetromino.rotation.to_string());
    }
}

fn update_tetrominoes(tetrominoes_list: &mut Vec<Tetromino>) {
    for tetromino in tetrominoes_list {
        tetromino.move_tetromino(0, 1);
    }
}

fn random_tetronimo() -> i32 {
    return rand::thread_rng().gen_range(1..=5) as i32;
}

fn random_tetromino_position() -> i32 {
    return rand::thread_rng().gen_range(2..WIDTH - 4) as i32;
}
