#![warn(missing_docs)]

//! Boggle game solver

mod results;

use std::path::Path;
use std::time::Instant;

use board::{Board, GameType};
use clap::{Parser, ValueEnum};
use dictionary::{Dictionary, WordSizeConstraint};
use numformat::NumFormat;
use solver::{find_words, SolverArgs};

use crate::results::print_results;

/// Boggle letters game solver
#[derive(Parser, Default)]
#[clap(author, version, about)]
struct Args {
    /// Game type
    #[clap(short = 'g', long = "game", default_value = "classic")]
    game_type: ArgsGameType,

    /// Board width
    #[clap(short = 'x', long = "width")]
    width: Option<u8>,

    /// Board height
    #[clap(short = 'y', long = "height")]
    height: Option<u8>,

    /// Word list file
    #[clap(
        short = 'd',
        long = "dictionary",
        default_value_t = default_dict().into(),
    )]
    dictionary_file: String,

    /// Minimum word length to find
    #[clap(short = 'm', long = "min-len", default_value_t = 3)]
    min_len: u8,

    /// Verbose output
    #[clap(short = 'v', long = "verbose")]
    verbose: bool,

    /// Debug output
    #[clap(long = "debug")]
    debug: bool,

    /// Dice faces to use. If none given a random board is generated for the game type.
    dice_faces: Vec<String>,
}

#[derive(ValueEnum, Debug, Clone, Default)]
enum ArgsGameType {
    #[default]
    Classic,
    New,
    BigOriginal,
    BigChallenge,
    BigDeluxe,
    Big2012,
    SuperBig,
}

impl std::fmt::Display for ArgsGameType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let args = Args::parse();

    // Check we have a dictionary
    if args.dictionary_file.is_empty() {
        eprintln!("No dictionary file given and none of the default dictionaries could be found.");
        eprintln!("Default dictionaries are:");

        for d in DICTS {
            eprintln!("  {d}");
        }

        std::process::exit(1);
    }

    // Load words
    let mut size = WordSizeConstraint::default();

    size.set_min(args.min_len as usize);

    let dictionary = Dictionary::new_from_file(&args.dictionary_file, size, args.verbose)?;

    let board = if args.dice_faces.is_empty() {
        // Convert arg game type to game type
        let game_type = match args.game_type {
            ArgsGameType::Classic => GameType::Classic,
            ArgsGameType::New => GameType::New,
            ArgsGameType::BigOriginal => GameType::BigOriginal,
            ArgsGameType::BigChallenge => GameType::BigChallenge,
            ArgsGameType::BigDeluxe => GameType::BigDeluxe,
            ArgsGameType::Big2012 => GameType::Big2012,
            ArgsGameType::SuperBig => GameType::SuperBig,
        };

        // Generate board
        Board::new_random(game_type)
    } else {
        Board::new(args.width, args.height, args.dice_faces)?
    };

    // Print board
    println!("Board:");
    board.print();

    // Find words
    let start_time = Instant::now();

    let words = find_words(SolverArgs {
        board: &board,
        dictionary: &dictionary,
        debug: args.debug,
    });

    if args.verbose {
        println!(
            "Search took {} seconds",
            start_time.elapsed().as_secs_f64().num_format_sigdig(2)
        );
    }

    // Print results
    print_results(words);

    Ok(())
}

const DICTS: [&str; 3] = [
    "words.txt",
    "words.txt.gz",
    "/etc/dictionaries-common/words",
];

fn default_dict() -> &'static str {
    DICTS
        .iter()
        .find(|d| dict_valid(d).is_some())
        .unwrap_or(&"")
}

fn dict_valid(dict: &str) -> Option<String> {
    if Path::new(dict).is_file() {
        Some(dict.into())
    } else {
        None
    }
}
