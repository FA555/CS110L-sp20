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
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn read_guess_char() -> char {
    io::stdout()
        .flush()
        .expect("Error flushing stdout.");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Error reading line.");

    if guess.len() != 2 {
        println!("Please enter a single character.");
        return read_guess_char();
    }

    guess.chars().next().unwrap()
}

fn find_in_word(secret_word_chars: &Vec<char>, guessed_word_chars: &Vec<char>, guess_char: char) -> Option<usize> {
    let mut i = 0;
    while i < secret_word_chars.len() {
        if guessed_word_chars[i] == '-' && secret_word_chars[i] == guess_char {
            return Some(i);
        }
        i += 1;
    }
    return None;
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
    let mut guessed_word_chars: Vec<char> = vec!['-'; secret_word_chars.len()];
    let mut guesses_left: u32 = NUM_INCORRECT_GUESSES;
    let mut guessed_letters: Vec<char> = Vec::new();
    let mut found: bool = false;

    while guesses_left > 0 {
        println!("The word so far is: {}", guessed_word_chars.iter().collect::<String>());
        println!("You have guessed the following letters: {}", guessed_letters.iter().collect::<String>());
        println!("You have {} guesses left.", guesses_left);
        print!("Please guess a letter: ");

        let guess_char = read_guess_char();
        guessed_letters.push(guess_char);

        if let Some(i) = find_in_word(&secret_word_chars, &guessed_word_chars, guess_char) {
            guessed_word_chars[i] = guess_char;
        } else {
            println!("Sorry, that letter is not in the word");
            guesses_left -= 1;
        }

        println!();
        if guessed_word_chars == secret_word_chars {
            found = true;
            break;
        }
    }

    if found {
        println!("Congratulations you guessed the secret word: {}!", secret_word);
    } else {
        println!("Sorry, you ran out of guesses!");
    }
}
