use rand::Rng;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const WORD_COUNT_PER_GAME: i32 = 132; //Should be divisible by 3

pub struct AllWords {
    pub all_words: Vec<String>,
}

impl AllWords {
    pub fn new() -> Self {
        Self {
            all_words: load_words(),
        }
    }
}

pub struct WordListIndex {
    pub current_index: usize,
}

pub struct WordList {
    pub list: Vec<String>,
}

impl WordList {
    pub fn new(word_list: Vec<String>) -> Self {
        Self {
            list: get_random_word_list(word_list),
        }
    }
}

pub struct PlayerWordList {
    pub list: Vec<String>,
}

impl PlayerWordList {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }
}

fn get_random_word_list(word_list: Vec<String>) -> Vec<String> {
    let total_word_count: usize = word_list.len();
    let mut words_for_game: Vec<String> = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..WORD_COUNT_PER_GAME {
        words_for_game.push(word_list[rng.gen_range(1..total_word_count)].clone());
    }

    words_for_game
}

pub fn load_words() -> Vec<String> {
    let mut all_words = Vec::new();

    if let Ok(lines) = read_lines("words.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(word) = line {
                all_words.push(word);
            }
        }
    } else if let Err(_) = read_lines("words.txt") {
        panic!("The words file could not be read; closing application")
    }
    all_words
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
