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
                '1' => DiceFace::Ligature("QU"),
                '2' => DiceFace::Ligature("IN"),
                '3' => DiceFace::Ligature("TH"),
                '4' => DiceFace::Ligature("ER"),
                '5' => DiceFace::Ligature("HE"),
                '6' => DiceFace::Ligature("AN"),
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
    /// Signel letter dice face
    Letter(char),
    /// Ligature dice face
    Ligature(&'static str),
    /// Word stop dice face
    Stop,
}

impl Display for DiceFace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Letter(c) => write!(f, "{:<2}", *c),
            Self::Ligature(s) => write!(f, "{s}"),
            Self::Stop => write!(f, "█ "),
        }
    }
}
