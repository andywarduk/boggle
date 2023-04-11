#![warn(missing_docs)]

//! Boggle board structures

use dictionary::uchar_to_elem_u8;
use rand::Rng;

mod dice;
mod gametype;

pub use dice::DiceFace;
pub use gametype::GameType;

/// Boggle board
pub struct Board {
    x: usize,
    y: usize,
    faces: Vec<Vec<DiceFace>>,
    dict_ents: Vec<Vec<Vec<u8>>>,
}

impl Board {
    /// Creates a new random Boggle board for the given game type
    pub fn new_random(game_type: GameType) -> Board {
        let (x, y) = game_type.layout();
        let mut dice = game_type.dice();

        let mut faces = Vec::with_capacity(y);
        let mut dict_ents = Vec::with_capacity(y);

        let mut rng = rand::thread_rng();

        for _ in 0..y {
            let mut faces_row = Vec::with_capacity(x);
            let mut dict_ents_row = Vec::with_capacity(x);

            for _ in 0..x {
                // Choose a dice
                let dice_elem = rng.gen_range(0..dice.len());
                let dice = dice.swap_remove(dice_elem);

                // Choose a face
                let face_elem = rng.gen_range(0..6);
                let face = dice.face(face_elem);

                // Add to dictionary entries
                dict_ents_row.push(match face {
                    DiceFace::Letter(c) => vec![uchar_to_elem_u8(c)],
                    DiceFace::Ligature(str) => str.chars().map(uchar_to_elem_u8).collect(),
                    DiceFace::Stop => vec![],
                });

                // Add to faces
                faces_row.push(face);
            }

            faces.push(faces_row);
            dict_ents.push(dict_ents_row);
        }

        Board {
            x,
            y,
            faces,
            dict_ents,
        }
    }

    /// Prints the Boggle board
    pub fn print(&self) {
        for y in 0..self.y {
            for x in 0..self.x {
                print!(" {}", self.faces[y][x]);
            }
            println!()
        }
    }

    /// Returns the dimensions of the Boggle board
    pub fn dimension(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    /// Returns the dice face at a given position on the Boggle board
    pub fn face(&self, x: usize, y: usize) -> DiceFace {
        self.faces[y][x].clone()
    }

    /// Returns the dictionary entry elements for a dice face on the Boggle board
    pub fn dict_ents(&self, x: usize, y: usize) -> &Vec<u8> {
        &self.dict_ents[y][x]
    }

    /// Returns a vector of dice neighbours on the Boggle board
    pub fn neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut result = Vec::new();

        if x > 0 {
            result.push((x - 1, y));
            if y > 0 {
                result.push((x - 1, y - 1));
            }
            if y < self.y - 1 {
                result.push((x - 1, y + 1));
            }
        }
        if y > 0 {
            result.push((x, y - 1));
        }

        if x < self.x - 1 {
            result.push((x + 1, y));
            if y < self.y - 1 {
                result.push((x + 1, y + 1));
            }
            if y > 0 {
                result.push((x + 1, y - 1));
            }
        }
        if y < self.y - 1 {
            result.push((x, y + 1));
        }

        result
    }
}
