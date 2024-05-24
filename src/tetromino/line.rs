use std::fmt;

use self::super::characters::TetrominoCharacter;

pub struct Line {
    pub x: i32,                              //Represents thes start of the line
    pub y: i32,                              // Represents the y position of the line
    pub characters: Vec<TetrominoCharacter>, // List of TetrominoCharacter
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
    pub fn new(x_pos: i32, y_pos: i32, num: i32) -> Line {
        Line {
            x: x_pos,
            y: y_pos,
            characters: Line::create_characters(x_pos, y_pos, num),
        }
    }

    /// Creates a list of TetrominoCharacter based on the given x and y position and the number of characterss
    pub fn create_characters(x_pos: i32, y_pos: i32, num: i32) -> Vec<TetrominoCharacter> {
        let mut characters: Vec<TetrominoCharacter> = Vec::new();

        for i in 0..num {
            characters.push(TetrominoCharacter::new(x_pos + i, y_pos, "[ ]"));
        }

        characters
    }

    /// Moves the line with its characters by the given x and y unitss
    pub fn move_line(&mut self, x_units: i32, y_units: i32) {
        self.x += x_units;
        self.y += y_units;

        for character in &mut self.characters {
            character.move_character(x_units, y_units);
        }
    }
}
