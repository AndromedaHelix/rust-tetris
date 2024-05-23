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

use std::fmt;
use std::thread;
use std::time::Duration;

const WIDTH: usize = 12; // 2 more to account for the borders
const HEIGHT: usize = 40;

/* Tetromino */
/// Represents a Tetromino character '[ ]' with its x and y position (Simulates a pixel)
#[derive(Copy, Clone)]
struct TetrominoCharacter {
    x: i32,
    y: i32,
    value: &'static str, // Use &'static str for string literals
}

impl TetrominoCharacter {
    fn new(x_pos: i32, y_pos: i32, value: &'static str) -> TetrominoCharacter {
        TetrominoCharacter {
            x: x_pos,
            y: y_pos,
            value,
        }
    }

    fn default() -> Self {
        TetrominoCharacter {
            x: 0,
            y: 0,
            value: "", // Default value, replace with actual default if needed
        }
    }

    fn move_character(&mut self, x_units: i32, y_units: i32) {
        self.x += x_units;
        self.y += y_units;
    }
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

    /// Creates a list of TetrominoCharacter based on the given x and y position and the number of characterss
    fn create_characters(x_pos: i32, y_pos: i32, num: i32) -> Vec<TetrominoCharacter> {
        let mut characters: Vec<TetrominoCharacter> = Vec::new();

        for i in 0..num {
            characters.push(TetrominoCharacter::new(x_pos + i, y_pos, "[ ]"));
        }

        characters
    }

    /// Moves the line with its characters by the given x and y unitss
    fn move_line(&mut self, x_units: i32, y_units: i32) {
        self.x += x_units;
        self.y += y_units;

        for character in &mut self.characters {
            character.move_character(x_units, y_units);
        }
    }
}

struct Tetromino {
    first_line: Line,
    second_line: Line,
    third_line: Line,
    fourth_line: Line,
    rotation: i32,
    shape_type: i32,
    stationary: bool,
}

impl Tetromino {
    fn new(first_line: Line, second_line: Line, shape_type: i32) -> Tetromino {
        Tetromino {
            first_line,
            second_line,
            third_line: Line::new(0, 0, 0),
            fourth_line: Line::new(0, 0, 0),
            rotation: 0,
            shape_type,
            stationary: false,
        }
    }

    fn move_tetromino(
        &mut self,
        x_units: i32,
        y_units: i32,
        game_borders: &mut [[bool; WIDTH]; HEIGHT + 1],
    ) {
        let collides: bool = self.collides(game_borders);

        if self.stationary {
            return;
        }

        let collides_horizontal = self.collides_horizontal(x_units);

        if collides {
            self.stationary = true;
            return;
        }

        if collides_horizontal {
            return;
        }

        self.first_line.move_line(x_units, y_units);
        self.second_line.move_line(x_units, y_units);
        self.third_line.move_line(x_units, y_units);
        self.fourth_line.move_line(x_units, y_units);
    }

    fn blank_tetromino(x_position: i32) -> Tetromino {
        Tetromino::new(Line::new(x_position, 1, 0), Line::new(0, 2, 0), 0)
    }

    /// Checks if the tetromino collides with the game borders
    ///
    /// # Arguments
    ///
    /// * `game_borders` - A vector containing the game borders
    ///
    /// # Returns
    ///
    /// A boolean indicating if the tetromino collides with the game borders
    fn collides(&mut self, game_borders: &mut [[bool; WIDTH]; HEIGHT + 1]) -> bool {
        let mut collides: bool = false;

        // Iterates through the y axis
        for y in 1..game_borders.len() - 1 {
            // Checks if the fourth line is empty (as it is not always used)
            if !self.fourth_line.to_string().is_empty() {
                // Iterates through the characters of the fourth line's characters
                for character in &self.fourth_line.characters {
                    // Iterates through the x axis
                    for x in 1..game_borders[y].len() {
                        // Checks if the characters X coordinate is the same as the game border's X coordinate, and if the game border at the fourth line's Y coordinate is true
                        if character.x == x as i32
                            && game_borders[(self.fourth_line.y + 1) as usize][x] == true
                        {
                            // collides is set to true
                            collides = true;
                            return collides;
                        }
                    }
                }
            }

            // Apply the same process for the rest of the lines
            if !self.third_line.to_string().is_empty() {
                for character in &self.third_line.characters {
                    for x in 1..game_borders[y].len() {
                        if character.x == x as i32
                            && game_borders[(self.third_line.y + 1) as usize][x] == true
                        {
                            collides = true;
                            return collides;
                        }
                    }
                }
            }

            if !self.second_line.to_string().is_empty() {
                for character in &self.second_line.characters {
                    for x in 1..game_borders[y].len() {
                        if character.x == x as i32
                            && game_borders[(self.second_line.y + 1) as usize][x] == true
                        {
                            collides = true;
                            return collides;
                        }
                    }
                }
            }

            // As the first line is always used, skip the check if it is empty
            for character in &self.first_line.characters {
                for x in 1..game_borders[y].len() {
                    if character.x == x as i32
                        && game_borders[(self.first_line.y + 1) as usize][x] == true
                    {
                        collides = true;
                        return collides;
                    }
                }
            }
        }

        return collides;
    }

    fn collides_horizontal(&mut self, x_units: i32) -> bool {
        let lines_array = [
            &self.first_line,
            &self.second_line,
            &self.third_line,
            &self.fourth_line,
        ];

        let max = lines_array
            .iter()
            .enumerate()
            .max_by_key(|&(_, line)| line.characters.len())
            .map(|(index, _)| index)
            .unwrap_or(0);

        let mut collides_horizontal = false;

        if let Some(line) = lines_array.get(max) {
            if line.x + x_units < 1
                || line.x + x_units > WIDTH as i32 - 1 - line.characters.len() as i32
            {
                collides_horizontal = true;
            }
        }

        collides_horizontal
    }

    fn clear(&mut self) {
        self.first_line.characters.clear();
        self.second_line.characters.clear();
        self.third_line.characters.clear();
        self.fourth_line.characters.clear();
    }

    fn rotate(&mut self, rotation: i32, game_borders: &mut [[bool; WIDTH]; HEIGHT + 1]) {
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

        let past_rotation = self.rotation;
        self.rotation = new_rotation;

        self.rotate_shape(self.rotation);

        if self.collides(game_borders) {
            self.rotation = past_rotation;
            self.rotate_shape(self.rotation);
        }

        if self.collides_horizontal(0) {
            self.rotation = past_rotation;
            self.rotate_shape(self.rotation);
        }
    }

    fn rotate_shape(&mut self, rotation: i32) {
        match rotation {
            90 => match self.shape_type {
                1 => {
                    self.clear();

                    self.first_line = Line::new(self.first_line.x, self.first_line.y, 1);
                    self.second_line = Line::new(self.first_line.x, self.first_line.y + 1, 1);
                    self.third_line = Line::new(self.first_line.x, self.first_line.y + 2, 1);
                    self.fourth_line = Line::new(self.first_line.x, self.first_line.y + 3, 1);
                }
                2 => {
                    self.clear();

                    self.first_line = Line::new(self.first_line.x, self.first_line.y, 2);
                    self.second_line = Line::new(self.first_line.x, self.first_line.y + 1, 1);
                    self.third_line = Line::new(self.first_line.x, self.first_line.y + 2, 1);
                }
                4 => {
                    self.clear();

                    self.first_line = Line::new(self.first_line.x, self.first_line.y, 1);
                    self.second_line = Line::new(self.first_line.x - 1, self.first_line.y + 1, 2);
                    self.third_line = Line::new(self.first_line.x - 1, self.first_line.y + 2, 1);
                }
                5 => {
                    self.clear();

                    self.first_line = Line::new(self.first_line.x + 1, self.first_line.y, 1);
                    self.second_line = Line::new(self.first_line.x, self.first_line.y + 1, 2);
                    self.third_line = Line::new(self.first_line.x, self.first_line.y + 2, 1);
                }
                _ => {}
            },
            180 => match self.shape_type {
                1 => {
                    self.clear();

                    self.first_line = Line::new(self.first_line.x, self.first_line.y, 4);
                }
                2 => {
                    self.clear();

                    self.first_line = Line::new(self.first_line.x, self.first_line.y, 1);
                    self.second_line = Line::new(self.first_line.x, self.first_line.y + 1, 3);
                }
                4 => {
                    self.clear();

                    self.first_line = Line::new(self.first_line.x, self.first_line.y, 2);
                    self.second_line = Line::new(self.first_line.x + 1, self.first_line.y + 1, 2);
                }
                5 => {
                    self.clear();

                    self.first_line = Line::new(self.first_line.x, self.first_line.y, 1);
                    self.second_line = Line::new(self.first_line.x - 1, self.first_line.y + 1, 3);
                }
                _ => {}
            },
            270 => match self.shape_type {
                1 => {
                    self.clear();

                    self.first_line = Line::new(self.first_line.x, self.first_line.y, 1);
                    self.second_line = Line::new(self.first_line.x, self.first_line.y + 1, 1);
                    self.third_line = Line::new(self.first_line.x, self.first_line.y + 2, 1);
                    self.fourth_line = Line::new(self.first_line.x, self.first_line.y + 3, 1);
                }
                2 => {
                    self.clear();

                    self.first_line = Line::new(self.first_line.x, self.first_line.y, 1);
                    self.second_line = Line::new(self.first_line.x, self.first_line.y + 1, 1);
                    self.third_line = Line::new(self.first_line.x, self.first_line.y + 2, 2);
                }
                4 => {
                    self.clear();

                    self.first_line = Line::new(self.first_line.x, self.first_line.y, 1);
                    self.second_line = Line::new(self.first_line.x - 1, self.first_line.y + 1, 2);
                    self.third_line = Line::new(self.first_line.x - 1, self.first_line.y + 2, 1);
                }
                5 => {
                    self.clear();

                    self.first_line = Line::new(self.first_line.x, self.first_line.y, 1);
                    self.second_line = Line::new(self.first_line.x - 1, self.first_line.y + 1, 2);
                    self.third_line = Line::new(self.first_line.x, self.first_line.y + 2, 1);

                    self.second_line.x = self.first_line.x - 1;
                    self.second_line.y = self.first_line.y + 1;
                    self.third_line.x = self.first_line.x;
                    self.third_line.y = self.first_line.y + 2;
                }
                _ => {}
            },
            0 => match self.shape_type {
                1 => {
                    self.clear();

                    self.first_line = Line::new(self.first_line.x, self.first_line.y, 4);
                }
                2 => {
                    self.clear();

                    self.first_line = Line::new(self.first_line.x, self.first_line.y, 3);
                    self.second_line = Line::new(self.first_line.x + 2, self.first_line.y + 1, 1);
                }
                4 => {
                    self.clear();

                    self.first_line = Line::new(self.first_line.x, self.first_line.y, 2);
                    self.second_line = Line::new(self.first_line.x + 1, self.first_line.y + 1, 2);
                }
                5 => {
                    self.clear();

                    self.first_line = Line::new(self.first_line.x - 1, self.first_line.y, 3);
                    self.second_line = Line::new(self.first_line.x + 1, self.first_line.y + 1, 1);
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
