#![warn(missing_docs)]

//! Boggle game solver

use std::collections::HashSet;

use board::{Board, DiceFace};
use dictionary::Dictionary;

/// Arguments for the Boggle solver
pub struct SolverArgs<'a> {
    /// String of letters to use (must be upper case A-Z)
    pub board: &'a Board,
    /// Dictionary to use
    pub dictionary: &'a Dictionary,
    /// Debug output
    pub debug: bool,
}

/// Find words in the dictionary on the board
pub fn find_words(args: SolverArgs) -> Vec<String> {
    let mut result = HashSet::new();

    // Vector of chosen letter elements
    let mut chosen = Vec::new();

    // Set of visited dice
    let mut visited = HashSet::new();

    // Start search recursion
    let (bx, by) = args.board.dimension();

    for y in 0..by {
        for x in 0..bx {
            if args.debug {
                println!("Starting at {x}x{y}");
            }

            find_words_rec(&args, &mut chosen, &mut visited, x, y, 0, &mut result);
        }
    }

    // Convert hash set to vector
    result.into_iter().collect()
}

fn find_words_rec(
    args: &SolverArgs,
    chosen: &mut Vec<u8>,
    visited: &mut HashSet<(usize, usize)>,
    x: usize,
    y: usize,
    mut dict_elem: u32,
    result: &mut HashSet<String>,
) {
    // Add to visited
    visited.insert((x, y));

    // Save chosen length
    let in_chosen = chosen.len();

    // Loop each face element
    for l in args.board.dict_ents(x, y) {
        dict_elem = args.dictionary.lookup_elem_letter_num(dict_elem, *l);

        if dict_elem == 0 {
            // No word found
            break;
        }

        // Add to chosen letters
        chosen.push(*l);

        if args.debug {
            debug_lookup(chosen, dict_elem);
        }
    }

    if dict_elem != 0 {
        if args.dictionary.elem_ends_word(dict_elem) {
            // Found word end - add to results
            result.insert(chosen_string(chosen));
        }

        // Recurse neighbours
        for (x, y) in args.board.neighbours(x, y) {
            // Don't process visited dice or stop face
            if !visited.contains(&(x, y)) && !matches!(args.board.face(x, y), DiceFace::Stop) {
                find_words_rec(args, chosen, visited, x, y, dict_elem, result);
            }
        }
    }

    // Remove from visited
    visited.remove(&(x, y));

    // SAFETY: length always decreasing and always removing the pushed entry/entries above
    unsafe {
        chosen.set_len(in_chosen);
    }
}

/// Converts chosen element vector to a string
#[inline]
fn chosen_string(chosen: &[u8]) -> String {
    chosen
        .iter()
        .map(|e| Dictionary::elem_to_uchar(*e))
        .collect::<String>()
}

/// Debug output for dictionary lookups
#[cold]
fn debug_lookup(chosen: &[u8], dict_elem: u32) {
    let string = chosen_string(chosen);
    let indent = string.len() - 1;

    println!("{:indent$}{} ({:?})", "", string, dict_elem);
}
