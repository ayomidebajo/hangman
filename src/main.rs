// use clap::{Arg, Command};
use rand::seq::SliceRandom;
use std::io;
// #[allow(dead_code)]
fn main() {
    struct PlayerRoot {
        word: String,
        no_of_guesses: i8,
        available_alphabets: Vec<char>,
        list_of_words_to_guess_from: Vec<String>,
        ouput_string: Vec<char>,
        max_tries: i8,
        guess: String,
        correct_guesses: Vec<char>,
    }

    impl PlayerRoot {
        fn new(
            word: &str,
            no_of_guesses: i8,
            available_alphabets: Vec<char>,
            list_of_words_to_guess_from: Vec<String>,
            ouput_string: Vec<char>,
            max: i8,
            guess: String,
            correct_guesses: Vec<char>,
        ) -> PlayerRoot {
            PlayerRoot {
                word: String::from(word),
                no_of_guesses,
                available_alphabets,
                list_of_words_to_guess_from,
                ouput_string,
                max_tries: max,
                guess,
                correct_guesses,
            }
        }

        fn generate_random_word(list: &Vec<String>) -> String {
            let word = list.choose(&mut rand::thread_rng()).unwrap();
            // println!("word {:?}", word);
            word.to_string()
        }
        //Still don't know if I should keep this function, it's basically for producing alpahbets
        fn _produce_alphabets(self) {
            let letters = String::from(
                "A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y,Z",
            );
            for item in letters.split(",") {
                println!("{:?}", item)
            }
        }
    }
    //list of words for the game
    let list_of_words = vec![
        "hunting".to_string(),
        "dizzy".to_string(),
        "while".to_string(),
        "string".to_string(),
        "something".to_string(),
        "notified".to_string(),
    ];
    let random_word = PlayerRoot::generate_random_word(&list_of_words);
    let letters = vec![
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    let guess_vec: Vec<char> = random_word.clone().chars().collect();
    println!("guess vec{:?}", guess_vec);
    let guess_chars = vec![];
    let ouput_string_vec = vec!['_'; guess_vec.len()];
    // ouput_string_vec[0] = guess_vec[0];
   println!("Welcome to the hangman game built with rust!, please enter a letter");
   println!("You already know what this is :D");
    println!("Fill in the blank spaces{:?}", ouput_string_vec);
    let mut player_one = PlayerRoot::new(
        &random_word,
        0,
        letters,
        list_of_words,
        ouput_string_vec,
        6,
        "".to_string(),
        guess_chars,
    );

    loop {
     

        //Takes in an input
        //todo unwrap the input or validate the input
        let mut guess = String::from("");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let altered_guess: char = guess.trim().chars().next().unwrap();
        player_one.no_of_guesses += 1;

        'outer: for n in player_one.word.char_indices() {
            if !player_one.correct_guesses.contains(&n.1) {
                if n.1 == altered_guess {
                    player_one.correct_guesses.push(n.1);
                    player_one.ouput_string[n.0] = n.1;
                    println!("Fill in the blank spaces{:?}", player_one.ouput_string);
                }
            } else {
                continue 'outer;
            }
        }
        if player_one.max_tries == player_one.no_of_guesses {
            println!("GAME OVER!!!");
            break;
        }
    }
}
