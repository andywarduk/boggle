use std::fmt::Display;

/// Dice structure
pub struct Dice {
    faces: Vec<DiceFace>,
}

impl Dice {
    /// Create a dice from a string of face characters (A-Z, 0-6)
    pub fn from_string(string: &str) -> Self {
        let faces = string
            .chars()
            .map(|c| match c {
                'A'..='Z' => DiceFace::Letter(c),
                '0' => DiceFace::Stop,
                '1' => DiceFace::Ligature("QU".to_string()),
                '2' => DiceFace::Ligature("IN".to_string()),
                '3' => DiceFace::Ligature("TH".to_string()),
                '4' => DiceFace::Ligature("ER".to_string()),
                '5' => DiceFace::Ligature("HE".to_string()),
                '6' => DiceFace::Ligature("AN".to_string()),
                _ => panic!("Unknown face value {c}"),
            })
            .collect();

        Dice { faces }
    }

    /// Returns a face of the dice
    pub fn face(&self, elem: usize) -> DiceFace {
        self.faces[elem].clone()
    }
}

/// Dice face enumeration
#[derive(Clone)]
pub enum DiceFace {
    /// Single letter dice face
    Letter(char),
    /// Ligature dice face
    Ligature(String),
    /// Word stop dice face
    Stop,
}

impl DiceFace {
    /// Converts a string to a dice face
    pub fn from_string(string: &str) -> Result<Self, String> {
        let chars = string
            .chars()
            .map(|c| {
                if !c.is_ascii_alphabetic() {
                    None
                } else {
                    Some(c.to_ascii_uppercase())
                }
            })
            .collect::<Option<Vec<_>>>();

        if let Some(chars) = chars {
            let face = match chars.len() {
                0 => DiceFace::Stop,
                1 => DiceFace::Letter(chars[0]),
                _ => DiceFace::Ligature(chars.iter().collect()),
            };

            Ok(face)
        } else {
            Err(format!("Invalid dice face: {string}"))
        }
    }
}

impl Display for DiceFace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = f.width().unwrap_or(1);

        match self {
            Self::Letter(c) => write!(f, "{:<width$}", *c),
            Self::Ligature(s) => write!(f, "{:<width$}", s),
            Self::Stop => write!(f, "{:<width$}", "█"),
        }
    }
}
