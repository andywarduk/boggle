#![warn(missing_docs)]

//! Word list and loader functions

use std::fs::{read_link, symlink_metadata, File};
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::path::PathBuf;
use std::time::Instant;

use flate2::bufread::GzDecoder;
use numformat::NumFormat;

/// Array of element numbers of the next dictionary level
type LetterVec = [u32; 27];

/// Dictionary structure
pub struct Dictionary {
    words: usize,
    tree: Vec<LetterVec>,
}

impl Dictionary {
    /// Loads a dictionary from a file
    pub fn new_from_file(file: &str, size: WordSizeConstraint, verbose: bool) -> io::Result<Self> {
        let path_buf = PathBuf::from(file);

        if verbose {
            println!("Loading words from file {}", Self::file_spec(&path_buf)?);
        }

        // Create buf reader for the file
        Self::new_from_bufread(&mut BufReader::new(File::open(&path_buf)?), size, verbose)
    }

    /// Loads a dictionary from a string
    #[allow(dead_code)]
    pub fn new_from_string(
        string: &str,
        size: WordSizeConstraint,
        verbose: bool,
    ) -> io::Result<Self> {
        if verbose {
            println!("Loading words from string '{string}'");
        }

        Self::new_from_bufread(&mut BufReader::new(string.as_bytes()), size, verbose)
    }

    /// Loads a dictionary from a byte array
    #[allow(dead_code)]
    pub fn new_from_bytes(
        bytes: &[u8],
        size: WordSizeConstraint,
        verbose: bool,
    ) -> io::Result<Self> {
        if verbose {
            println!("Loading words from byte array (length {})", bytes.len());
        }

        Self::new_from_bufread(&mut BufReader::new(bytes), size, verbose)
    }

    /// Loads a dictionary from an entity implementing BufRead
    /// Handles gzip compressed buffers
    pub fn new_from_bufread(
        bufread: &mut dyn BufRead,
        size: WordSizeConstraint,
        verbose: bool,
    ) -> io::Result<Self> {
        // Get start time
        let start_time = Instant::now();

        // Fill the bufreader buffer
        let buf = bufread.fill_buf()?;

        // Check for gzip signature
        if buf.len() >= 2 && buf[0] == 0x1f && buf[1] == 0x8b {
            // gzip compressed file
            if verbose {
                println!("Decompressing word list");
            }

            Self::new_from_bufread_internal(
                start_time,
                &mut BufReader::new(GzDecoder::new(bufread)),
                size,
                verbose,
            )
        } else {
            Self::new_from_bufread_internal(start_time, bufread, size, verbose)
        }
    }

    /// Loads a dictionary from an entity implementing BufRead
    fn new_from_bufread_internal(
        start_time: Instant,
        bufread: &mut dyn BufRead,
        size: WordSizeConstraint,
        verbose: bool,
    ) -> io::Result<Self> {
        let mut tree = Vec::new();

        let empty = [0; 27];

        let mut lines: usize = 0;
        let mut words: usize = 0;
        let mut too_short: usize = 0;
        let mut too_long: usize = 0;
        let mut wrong_case: usize = 0;

        tree.push(empty);

        // Iterate file lines
        for line in bufread.lines() {
            let line = line?;

            lines += 1;

            // Check length
            let length = line.len();

            if length > size.max {
                too_long += 1;
                continue;
            }

            if length < size.min {
                too_short += 1;
                continue;
            }

            // Make sure word consists of all lower case ascii characters
            if !Self::is_ascii_lower(&line) {
                wrong_case += 1;
                continue;
            }

            // Add this word to the tree
            words += 1;

            let mut cur_elem = 0;

            for c in line.chars() {
                let letter: usize = Dictionary::lchar_to_elem(c);

                cur_elem = if tree[cur_elem][letter] == 0 {
                    tree.push(empty);
                    let e = tree.len() - 1;
                    tree[cur_elem][letter] = e as u32;
                    e
                } else {
                    tree[cur_elem][letter] as usize
                };
            }

            // Mark end of word
            tree[cur_elem][0] = 1;
        }

        let dictionary = Self { words, tree };

        if verbose {
            println!(
                "Dictionary read in {} seconds",
                start_time.elapsed().as_secs_f64().num_format_sigdig(2)
            );

            println!(
                "{} total words, ({} too short, {} too long, {} not all lower case)",
                lines.num_format(),
                too_short.num_format(),
                too_long.num_format(),
                wrong_case.num_format()
            );

            println!(
                "Dictionary words {}, tree nodes {} ({} bytes of {} allocated)",
                dictionary.word_count().num_format(),
                dictionary.tree_node_count().num_format(),
                dictionary.tree_mem_usage().num_format(),
                dictionary.tree_mem_alloc().num_format(),
            );
        }

        Ok(dictionary)
    }

    /// Returns the number of words stored in the dictionary
    pub fn word_count(&self) -> usize {
        self.words
    }

    /// Returns the size of the dictionary tree
    pub fn tree_node_count(&self) -> usize {
        self.tree.len()
    }

    /// Returns the used memory of the dictionary tree in bytes
    pub fn tree_mem_usage(&self) -> usize {
        self.tree_node_count() * std::mem::size_of::<LetterVec>()
    }

    /// Returns the allocated memory of the dictionary tree in bytes
    pub fn tree_mem_alloc(&self) -> usize {
        self.tree.capacity() * std::mem::size_of::<LetterVec>()
    }

    /// Determines if the letter ends the word
    #[inline]
    pub fn elem_ends_word(&self, elem: u32) -> bool {
        self.tree[elem as usize][0] != 0
    }

    /// Looks up the letter number (1-26) in the dictionary tree node
    #[inline]
    pub fn lookup_elem_letter_num(&self, elem: u32, letter: u8) -> u32 {
        self.tree[elem as usize][letter as usize]
    }

    #[inline]
    /// Converts a lower case character into a dictionary entry index (usize)
    fn lchar_to_elem<T>(c: char) -> T
    where
        T: std::convert::From<u8>,
    {
        (c as u8 - (b'a' - 1)).into()
    }

    #[inline]
    /// Converts an upper case character into a dictionary entry index (u8)
    pub fn uchar_to_elem<T>(c: char) -> T
    where
        T: std::convert::From<u8>,
    {
        (c as u8 - (b'A' - 1)).into()
    }

    #[inline]
    /// Converts dictionary entry index (u8) into an upper case character
    pub fn elem_to_uchar(e: u8) -> char {
        (e + b'A' - 1) as char
    }

    /// Follows symlinks in a path returning the followed paths as a string
    fn file_spec(path: &PathBuf) -> io::Result<String> {
        let meta = symlink_metadata(path)?;

        if meta.is_symlink() {
            let target = read_link(path)?;

            Ok(format!(
                "{} -> {}",
                path.to_string_lossy(),
                Self::file_spec(&target)?
            ))
        } else {
            Ok(format!("{}", path.to_string_lossy()))
        }
    }

    #[inline]
    /// Returns true if the passed string is all lower case
    fn is_ascii_lower(s: &str) -> bool {
        s.chars().all(|c| c.is_ascii_lowercase())
    }
}

/// Word size constraints to use when loading a dictionary
pub struct WordSizeConstraint {
    min: usize,
    max: usize,
}

impl WordSizeConstraint {
    /// Sets the minimum length for a word
    pub fn set_min(&mut self, min: usize) {
        self.min = min;
    }

    /// Sets the maximum length for a word
    pub fn set_max(&mut self, max: usize) {
        self.max = max;
    }
}

impl Default for WordSizeConstraint {
    fn default() -> Self {
        Self {
            min: 0,
            max: usize::MAX,
        }
    }
}

#[cfg(test)]
mod tests {
    use flate2::write::GzEncoder;
    use flate2::Compression;

    use super::*;

    fn gz_dict(string: &str) -> Vec<u8> {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(string.as_bytes()).unwrap();
        encoder.finish().unwrap()
    }

    #[test]
    fn dict1() {
        // Create dictionary with one word in it "rust"
        let dictionary = Dictionary::new_from_string("rust", Default::default(), false).unwrap();

        test_dict1(dictionary)
    }

    #[test]
    fn dict1z() {
        // Create dictionary from compressed data with one word in it "rust"
        let dictionary =
            Dictionary::new_from_bytes(&gz_dict("rust"), Default::default(), false).unwrap();

        test_dict1(dictionary)
    }

    fn test_dict1(dictionary: Dictionary) {
        assert_eq!(dictionary.word_count(), 1);
        assert_eq!(dictionary.tree_node_count(), 5);
        assert_eq!(dictionary.tree_mem_usage(), 5 * 27 * 4);

        assert_eq!(
            dictionary.lookup_elem_letter_num(0, Dictionary::uchar_to_elem('R')),
            1
        );
        assert_eq!(
            dictionary.lookup_elem_letter_num(1, Dictionary::uchar_to_elem('U')),
            2
        );
        assert_eq!(
            dictionary.lookup_elem_letter_num(2, Dictionary::uchar_to_elem('S')),
            3
        );
        assert_eq!(
            dictionary.lookup_elem_letter_num(3, Dictionary::uchar_to_elem('T')),
            4
        );
        assert!(dictionary.elem_ends_word(4));
    }

    #[test]
    fn dict2() {
        // Create dictionary with two words, "rust" and "rusty"
        let dictionary =
            Dictionary::new_from_string("rust\nrusty", Default::default(), false).unwrap();

        test_dict2(dictionary);
    }

    #[test]
    fn dict2z() {
        // Create dictionary from compressed data with two words, "rust" and "rusty"
        let dictionary =
            Dictionary::new_from_bytes(&gz_dict("rust\nrusty"), Default::default(), false).unwrap();

        test_dict2(dictionary);
    }

    fn test_dict2(dictionary: Dictionary) {
        assert_eq!(dictionary.word_count(), 2);
        assert_eq!(dictionary.tree_node_count(), 6);
        assert_eq!(dictionary.tree_mem_usage(), 6 * 4 * 27);

        assert_eq!(
            dictionary.lookup_elem_letter_num(0, Dictionary::uchar_to_elem('R')),
            1
        );
        assert_eq!(
            dictionary.lookup_elem_letter_num(1, Dictionary::uchar_to_elem('U')),
            2
        );
        assert_eq!(
            dictionary.lookup_elem_letter_num(2, Dictionary::uchar_to_elem('S')),
            3
        );
        assert_eq!(
            dictionary.lookup_elem_letter_num(3, Dictionary::uchar_to_elem('T')),
            4
        );
        assert!(dictionary.elem_ends_word(4));
        assert_eq!(
            dictionary.lookup_elem_letter_num(4, Dictionary::uchar_to_elem('Y')),
            5
        );
        assert!(dictionary.elem_ends_word(5));
    }
}
