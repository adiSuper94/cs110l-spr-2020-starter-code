// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate console;
extern crate rand;

use console::Term;
use rand::Rng;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    // println!("random word: {}", secret_word);

    // Your code here! :)
    let mut word_map: HashMap<char, Vec<usize>> = HashMap::new();
    let mut word_so_far: Vec<char> = Vec::new();
    for i in 0..secret_word.len() {
        let key: char = secret_word_chars[i];
        word_map.insert(key, Vec::new());
        word_so_far.push('-');
    }
    for i in 0..secret_word.len() {
        let key: char = secret_word_chars[i];
        word_map.get_mut(&key).unwrap().push(i);
    }
    let mut guessed_chars: HashSet<char> = HashSet::new();
    let mut guesses_left = 5;
    loop {
        if guesses_left <= 0 {
            break;
            println!("Sorry, you ran out of guesses!");
        }
        println!("The word so far is: {:?}", word_so_far);
        println!(
            "You have guessed the following letters: {:?}",
            guessed_chars
        );
        println!("You have {} guesses left", guesses_left);
        println!("Please guess a letter: ");
        let guess: char = Term::stdout().read_char().unwrap();
        guessed_chars.insert(guess);
        if !word_map.contains_key(&guess) {
            guesses_left -= 1;
            println!("Sorry, that letter is not in the word");
        } else {
            let idx = word_map.get(&guess).unwrap();
            for &id in idx {
                word_so_far[id] = guess;
            }
        }
    }
}
