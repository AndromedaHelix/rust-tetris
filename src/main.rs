/*

__          __   _ _   _               _                  _                     _____      _     _          _____       _   _
\ \        / /  (_) | | |             | |                | |                   |  __ \    | |   | |        / ____|     | | (_)
 \ \  /\  / / __ _| |_| |_ ___ _ __   | |__  _   _       | |_   _  __ _ _ __   | |__) |_ _| |__ | | ___   | |  __ _   _| |_ _  ___ _ __ _ __ ___ ____
  \ \/  \/ / '__| | __| __/ _ \ '_ \  | '_ \| | | |  _   | | | | |/ _` | '_ \  |  ___/ _` | '_ \| |/ _ \  | | |_ | | | | __| |/ _ \ '__| '__/ _ \_  /
   \  /\  /| |  | | |_| ||  __/ | | | | |_) | |_| | | |__| | |_| | (_| | | | | | |  | (_| | |_) | | (_) | | |__| | |_| | |_| |  __/ |  | | |  __// /
    \/  \/ |_|  |_|\__|\__\___|_| |_| |_.__/ \__, |  \____/ \__,_|\__,_|_| |_| |_|   \__,_|_.__/|_|\___/   \_____|\__,_|\__|_|\___|_|  |_|  \___/___|
                                              __/ |
                                             |___/
 */

use rand::Rng;
use std::io::Read;

extern crate termion;
use std::io::{stdout, Write};
use termion::raw::IntoRawMode;
use termion::{async_stdin, clear};

use std::thread;
use std::time::Duration;

mod tetromino;
use tetromino::tetromino::Tetromino;
use tetromino::characters::TetrominoCharacter;
use tetromino::line::Line;

const WIDTH: usize = 12; // 2 more to account for the borders
const HEIGHT: usize = 40;

/* Game loop */

fn main() {
    let mut screen: [[&str; WIDTH]; HEIGHT] = [[""; WIDTH]; HEIGHT];
    let mut rendered_tetrominoes_list: Vec<Tetromino> = Vec::new();
    let mut unrendered_tetrominoes_list: Vec<Tetromino> = Vec::new();

    let mut game_borders: [[bool; WIDTH]; HEIGHT + 1] = [[false; WIDTH]; HEIGHT + 1];
    for x in 0..WIDTH {
        game_borders[HEIGHT][x] = true;
    }

    // When the tetrominoes' stationary state is reached, they are added to the built_tetrominoes array
    // and removed from the unredered_tetrominoes_list
    //
    // The built_tetrominoes array is used to check for collisions with the tetrominoes that have already
    // been built. Also, the built_tetrominoes array is used to render the tetrominoes that have already
    // been built and to handle the scoring system.
    //
    // Also helps in reducing the number of computations per second needed to render moving tetrominoes as the unredered_tetrominoes_list reduces in size

    let mut built_tetrominoes: [[TetrominoCharacter; WIDTH]; HEIGHT] =
        [[TetrominoCharacter::default(); WIDTH]; HEIGHT];

    create_screen(&mut screen);
    create_tetronimo(&mut unrendered_tetrominoes_list);

    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    let mut movement_counter = 1;

    let mut score: u32 = 0;

    writeln!(stdout, "{}{}", clear::All, termion::cursor::Hide).unwrap();
    display_screen(
        &screen,
        &mut unrendered_tetrominoes_list,
        &mut rendered_tetrominoes_list,
        &mut stdout,
        &built_tetrominoes,
        score,
    );

    loop {
        write!(stdout, "{}", termion::clear::CurrentLine).unwrap();

        let b = stdin.next();

        if let Some(Ok(b'q')) = b {
            break;
        }
        if let Some(Ok(b'a')) = b {
            move_tetrmonioes(&mut unrendered_tetrominoes_list, -1, &mut game_borders);

            display_screen(
                &screen,
                &mut unrendered_tetrominoes_list,
                &mut rendered_tetrominoes_list,
                &mut stdout,
                &built_tetrominoes,
                score,
            );
        }
        if let Some(Ok(b'd')) = b {
            move_tetrmonioes(&mut unrendered_tetrominoes_list, 1, &mut game_borders);
            display_screen(
                &screen,
                &mut unrendered_tetrominoes_list,
                &mut rendered_tetrominoes_list,
                &mut stdout,
                &built_tetrominoes,
                score,
            );
        }
        if let Some(Ok(b'r')) = b {
            rotate_tetrominoes(&mut unrendered_tetrominoes_list, 90, &mut game_borders);
            display_screen(
                &screen,
                &mut unrendered_tetrominoes_list,
                &mut rendered_tetrominoes_list,
                &mut stdout,
                &built_tetrominoes,
                score,
            );
        }

        thread::sleep(Duration::from_millis(100));

        let mut create: bool = true;

        for tetromino in &unrendered_tetrominoes_list {
            if !tetromino.stationary {
                create = false;
                break;
            }
        }

        if create {
            create_tetronimo(&mut unrendered_tetrominoes_list);
        }

        if movement_counter % 2 == 0 {
            update_tetrominoes(&mut unrendered_tetrominoes_list, &mut game_borders);
        }
        movement_counter += 1;

        display_screen(
            &screen,
            &mut unrendered_tetrominoes_list,
            &mut rendered_tetrominoes_list,
            &mut stdout,
            &built_tetrominoes,
            score,
        );

        move_to_built(&mut unrendered_tetrominoes_list, &mut built_tetrominoes);

        check_complete_line(&mut built_tetrominoes, &mut score);

        remake_gameborders(&mut game_borders, &mut built_tetrominoes);

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
    built_tetroinoes: &[[TetrominoCharacter; WIDTH]; HEIGHT],
    score: u32,
) {
    writeln!(stdout, "{}{}", clear::All, termion::cursor::Hide).unwrap();

    writeln!(stdout, "Score: {}", score).unwrap();

    write!(stdout, "\n\r").unwrap();

    for i in 0..HEIGHT {
        let mut j = 0;
        while j < WIDTH {
            let mut found_tetromino = false;
            let mut x = 0;

            if built_tetroinoes[i][j].x == j as i32
                && built_tetroinoes[i][j].y == i as i32
                && !built_tetroinoes[i][j].value.is_empty()
            {
                write!(stdout, "{}", built_tetroinoes[i][j].value).unwrap();
                j += 1;
                continue;
            }

            while !unrendered_tetrominoes.is_empty() && x < unrendered_tetrominoes.len() {
                let tetromino = &unrendered_tetrominoes[x];

                let skip_distance_first = (tetromino.first_line.characters.len()) as usize;
                let skip_distance_second = (tetromino.second_line.characters.len()) as usize;

                let skip_distance_third = (tetromino.third_line.characters.len()) as usize;

                let skip_distance_fourth = (tetromino.fourth_line.characters.len()) as usize;

                if tetromino.first_line.x as usize == j && tetromino.first_line.y as usize == i {
                    write!(stdout, "{}", tetromino.first_line).unwrap();
                    stdout.flush().unwrap();
                    j += skip_distance_first;
                    found_tetromino = true;
                    if tetromino.second_line.to_string().is_empty() {
                        rendered_tetrominoes.push(unrendered_tetrominoes.remove(x));
                    }
                    break;
                } else if tetromino.second_line.x as usize == j
                    && tetromino.second_line.y as usize == i
                    && !tetromino.second_line.to_string().is_empty()
                {
                    found_tetromino = true;
                    j += skip_distance_second;
                    write!(stdout, "{}", tetromino.second_line).unwrap();
                    stdout.flush().unwrap();
                    if tetromino.third_line.to_string().is_empty() {
                        rendered_tetrominoes.push(unrendered_tetrominoes.remove(x));
                    }
                    break;
                } else if tetromino.third_line.x as usize == j
                    && tetromino.third_line.y as usize == i
                    && !tetromino.third_line.to_string().is_empty()
                {
                    found_tetromino = true;
                    j += skip_distance_third;
                    write!(stdout, "{}", tetromino.third_line).unwrap();
                    stdout.flush().unwrap();
                    if tetromino.fourth_line.to_string().is_empty() {
                        rendered_tetrominoes.push(unrendered_tetrominoes.remove(x));
                    }
                    break;
                } else if tetromino.shape_type == 1
                    && tetromino.fourth_line.x as usize == j
                    && tetromino.fourth_line.y as usize == i
                    && !tetromino.fourth_line.to_string().is_empty()
                {
                    found_tetromino = true;
                    j += skip_distance_fourth;
                    write!(stdout, "{}", tetromino.fourth_line).unwrap();
                    stdout.flush().unwrap();
                    rendered_tetrominoes.push(unrendered_tetrominoes.remove(x));
                    break;
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

fn move_to_built(
    tetrominoes_list: &mut Vec<Tetromino>,
    built_tetrominoes: &mut [[TetrominoCharacter; WIDTH]; HEIGHT],
) {
    let mut i = 0;
    while i < tetrominoes_list.len() {
        if tetrominoes_list[i].stationary {
            let tetromino = tetrominoes_list.remove(i);

            for character in tetromino.first_line.characters {
                let x = character.x;
                let y = character.y;
                built_tetrominoes[y as usize][x as usize] = character;
            }
            for character in tetromino.second_line.characters {
                let x = character.x;
                let y = character.y;
                built_tetrominoes[y as usize][x as usize] = character;
            }
            for character in tetromino.third_line.characters {
                let x = character.x;
                let y = character.y;
                built_tetrominoes[y as usize][x as usize] = character;
            }
            for character in tetromino.fourth_line.characters {
                let x = character.x;
                let y = character.y;
                built_tetrominoes[y as usize][x as usize] = character;
            }
        } else {
            i += 1;
        }
    }
}

fn check_complete_line(
    built_tetrominoes: &mut [[TetrominoCharacter; WIDTH]; HEIGHT],
    score: &mut u32,
) -> bool {
    let mut complete_line = false;
    let mut lines_cleared = 0;

    for i in 0..HEIGHT {
        let mut line_is_complete = true;

        for j in 1..WIDTH - 1 {
            if built_tetrominoes[i][j].value.is_empty() {
                line_is_complete = false;
                break;
            }
        }

        if line_is_complete {
            complete_line = true;
            lines_cleared += 1;

            // Clear the current line
            for j in 0..WIDTH {
                built_tetrominoes[i][j] = TetrominoCharacter::default();
            }

            // Shift all rows above the current line down by one row
            for k in (1..=i).rev() {
                for j in 0..WIDTH {
                    built_tetrominoes[k][j] = built_tetrominoes[k - 1][j];
                    // Move character down by one row
                    built_tetrominoes[k][j].move_character(0, 1);
                }
            }

            // Clear the top row since it has been shifted down
            for j in 0..WIDTH {
                built_tetrominoes[0][j] = TetrominoCharacter::default();
            }
        }
    }

    // Update score based on the number of lines cleared
    *score += lines_cleared * 100;

    complete_line
}

fn remake_gameborders(
    game_borders: &mut [[bool; WIDTH]; HEIGHT + 1],
    built_tetrominoes: &mut [[TetrominoCharacter; WIDTH]; HEIGHT],
) {
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            if built_tetrominoes[i][j].value.is_empty() {
                game_borders[i][j] = false;
            } else {
                game_borders[i][j] = true;
            }
        }
    }
}

fn create_tetronimo(tetrominoes_list: &mut Vec<Tetromino>) {
    let random_number: i32 = random_tetronimo();
    let x_position: i32 = random_tetromino_position();
    let mut tetromino_shape: Tetromino = Tetromino::blank_tetromino(x_position);

    match random_number {
        1 => {
            tetromino_shape.first_line.characters =
                Line::create_characters(x_position, tetromino_shape.first_line.y, 4);
            tetromino_shape.shape_type = 1;
        }
        2 => {
            tetromino_shape.first_line.characters =
                Line::create_characters(x_position, tetromino_shape.first_line.y, 3);
            tetromino_shape.second_line.characters =
                Line::create_characters(x_position + 2, tetromino_shape.first_line.y + 1, 1);
            tetromino_shape.second_line.x = x_position + 2;
            tetromino_shape.shape_type = 2;
        }
        3 => {
            tetromino_shape.first_line.characters =
                Line::create_characters(x_position, tetromino_shape.first_line.y, 2);
            tetromino_shape.second_line.characters =
                Line::create_characters(x_position, tetromino_shape.first_line.y + 1, 2);
            tetromino_shape.second_line.x = x_position;
            tetromino_shape.shape_type = 3;
        }
        4 => {
            tetromino_shape.first_line.characters =
                Line::create_characters(x_position, tetromino_shape.first_line.y, 2);
            tetromino_shape.second_line.characters =
                Line::create_characters(x_position + 1, tetromino_shape.first_line.y + 1, 2);
            tetromino_shape.second_line.x = x_position + 1;
            tetromino_shape.shape_type = 4;
        }
        5 => {
            tetromino_shape.first_line.characters =
                Line::create_characters(x_position, tetromino_shape.first_line.y, 3);
            tetromino_shape.second_line.characters =
                Line::create_characters(x_position + 1, tetromino_shape.first_line.y + 1, 1);
            tetromino_shape.second_line.x = x_position + 1;
            tetromino_shape.shape_type = 5;
        }
        _ => panic!("Invalid tetromino shape"),
    }

    tetrominoes_list.push(tetromino_shape);
}

fn move_tetrmonioes(
    tetrominoes_list: &mut Vec<Tetromino>,
    movement: i32,
    game_borders: &mut [[bool; WIDTH]; HEIGHT + 1],
) {
    for tetromino in tetrominoes_list {
        tetromino.move_tetromino(movement, 0, game_borders);
    }
}

fn rotate_tetrominoes(
    tetrominoes_list: &mut Vec<Tetromino>,
    rotation: i32,
    game_borders: &mut [[bool; WIDTH]; HEIGHT + 1],
) {
    for tetromino in tetrominoes_list {
        tetromino.rotate(rotation, game_borders);
    }
}

fn update_tetrominoes(
    tetrominoes_list: &mut Vec<Tetromino>,
    game_borders: &mut [[bool; WIDTH]; HEIGHT + 1],
) {
    for tetromino in tetrominoes_list {
        tetromino.move_tetromino(0, 1, game_borders);
    }
}

/* Random helper methods */

fn random_tetronimo() -> i32 {
    return rand::thread_rng().gen_range(1..=5) as i32;
}

fn random_tetromino_position() -> i32 {
    return rand::thread_rng().gen_range(2..WIDTH - 4) as i32;
}
