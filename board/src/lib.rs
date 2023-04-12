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
    /// Creates a new board from given optional dimensions and dice faces
    pub fn new(
        width: Option<u8>,
        height: Option<u8>,
        dice_faces: Vec<String>,
    ) -> Result<Self, String> {
        let x;
        let y;

        // Calculate board dimensions
        if let Some(width) = width {
            x = width as usize;

            if let Some(height) = height {
                y = height as usize;
            } else {
                let calc_y = dice_faces.len() as f32 / x as f32;

                if calc_y.fract() == 0.0 {
                    y = calc_y as usize;
                } else {
                    y = 0;
                    Err("Unable to calculate board height from width and number of dice faces")?
                }
            }
        } else if let Some(height) = height {
            y = height as usize;

            let calc_x = dice_faces.len() as f32 / y as f32;

            if calc_x.fract() == 0.0 {
                x = calc_x as usize;
            } else {
                x = 0;
                Err("Unable to calculate board width from height and number of dice faces")?
            }
        } else {
            let calc_dim = (dice_faces.len() as f32).sqrt();

            if calc_dim.fract() == 0.0 {
                x = calc_dim as usize;
                y = calc_dim as usize;
            } else {
                x = 0;
                y = 0;
                Err("Unable to calculate board size from number of dice faces")?
            }
        }

        // Get flat vector of dice faces
        let faces_flat = dice_faces
            .iter()
            .map(|f| DiceFace::from_string(f))
            .collect::<Result<Vec<_>, String>>()?;

        // Build 2-d vector of dice faces
        let faces = faces_flat
            .chunks(x)
            .map(|row| row.to_vec())
            .collect::<Vec<_>>();

        // Build dictionary elements vector
        let dict_ents = Self::build_dict_ents(&faces);

        Ok(Self {
            x,
            y,
            faces,
            dict_ents,
        })
    }

    /// Creates a new random Boggle board for the given game type
    pub fn new_random(game_type: GameType) -> Board {
        let (x, y) = game_type.layout();
        let mut dice = game_type.dice();

        let mut faces = Vec::with_capacity(y);

        let mut rng = rand::thread_rng();

        for _ in 0..y {
            let mut faces_row = Vec::with_capacity(x);

            for _ in 0..x {
                // Choose a dice
                let dice_elem = rng.gen_range(0..dice.len());
                let dice = dice.swap_remove(dice_elem);

                // Choose a face
                let face_elem = rng.gen_range(0..6);
                let face = dice.face(face_elem);

                // Add to faces
                faces_row.push(face);
            }

            faces.push(faces_row);
        }

        // Build dictionary elements vector
        let dict_ents = Self::build_dict_ents(&faces);

        Board {
            x,
            y,
            faces,
            dict_ents,
        }
    }

    /// Prints the Boggle board
    pub fn print(&self) {
        let longest = self
            .faces
            .iter()
            .flatten()
            .map(|f| match f {
                DiceFace::Ligature(s) => s.len(),
                _ => 1,
            })
            .max()
            .unwrap_or(1);

        for y in 0..self.y {
            for x in 0..self.x {
                print!(" {:<longest$}", self.faces[y][x]);
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

    /// Builds a 3d vector of dictionary entries from a 2d vector of dice faces
    fn build_dict_ents(faces: &[Vec<DiceFace>]) -> Vec<Vec<Vec<u8>>> {
        faces
            .iter()
            .map(|row| {
                row.iter()
                    .map(|face| match face {
                        DiceFace::Letter(c) => vec![uchar_to_elem_u8(*c)],
                        DiceFace::Ligature(str) => str.chars().map(uchar_to_elem_u8).collect(),
                        DiceFace::Stop => vec![],
                    })
                    .collect()
            })
            .collect()
    }
}
