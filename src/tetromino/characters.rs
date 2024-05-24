/* Tetromino */
/// Represents a Tetromino character '[ ]' with its x and y position (Simulates a pixel)
#[derive(Copy, Clone)]
pub struct TetrominoCharacter {
    pub x: i32,
    pub y: i32,
    pub value: &'static str, // Use &'static str for string literals
}

impl TetrominoCharacter {
    pub fn new(x_pos: i32, y_pos: i32, value: &'static str) -> TetrominoCharacter {
        TetrominoCharacter {
            x: x_pos,
            y: y_pos,
            value,
        }
    }

    pub fn default() -> Self {
        TetrominoCharacter {
            x: 0,
            y: 0,
            value: "", // Default value, replace with actual default if needed
        }
    }

    pub fn move_character(&mut self, x_units: i32, y_units: i32) {
        self.x += x_units;
        self.y += y_units;
    }
}
