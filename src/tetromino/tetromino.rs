use super::line::Line;

const WIDTH: usize = 12; // 2 more to account for the borders
const HEIGHT: usize = 40;

pub struct Tetromino {
    pub first_line: Line,
    pub second_line: Line,
    pub third_line: Line,
    pub fourth_line: Line,
    pub rotation: i32,
    pub shape_type: i32,
    pub stationary: bool,
}

impl Tetromino {
    pub fn new(first_line: Line, second_line: Line, shape_type: i32) -> Tetromino {
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

    pub fn move_tetromino(
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

    pub fn blank_tetromino(x_position: i32) -> Tetromino {
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
    pub fn collides(&mut self, game_borders: &mut [[bool; WIDTH]; HEIGHT + 1]) -> bool {
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

    pub fn collides_horizontal(&mut self, x_units: i32) -> bool {
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

    pub fn clear(&mut self) {
        self.first_line.characters.clear();
        self.second_line.characters.clear();
        self.third_line.characters.clear();
        self.fourth_line.characters.clear();
    }

    pub fn rotate(&mut self, rotation: i32, game_borders: &mut [[bool; WIDTH]; HEIGHT + 1]) {
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

    pub fn rotate_shape(&mut self, rotation: i32) {
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
