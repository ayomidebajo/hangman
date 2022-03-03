use clap::{Arg, Command};
use rand::seq::SliceRandom;
use std::io;

fn main() {
    struct PlayerRoot {
        word: String,
        no_of_guesses: i8,
        available_alphabets: Vec<char>,
        list_of_words_to_guess_from: Vec<String>,
        score: i8,
        max_tries: i8,
        guess: String,
        correct_guesses: Vec<char>,
    }

    struct PlayerGuesser {
        guess: char,
        tries: i8,
    }

    impl PlayerRoot {
        fn new(
            word: &str,
            no_of_guesses: i8,
            available_alphabets: Vec<char>,
            list_of_words_to_guess_from: Vec<String>,
            _score: i8,
            max: i8,
            guess: String,
            correct_guesses: Vec<char>,
        ) -> PlayerRoot {
            PlayerRoot {
                word: String::from(word),
                no_of_guesses,
                available_alphabets,
                list_of_words_to_guess_from,
                score: 0,
                max_tries: max,
                guess,
                correct_guesses,
            }
        }

        fn generate_random_word(list: &Vec<String>) -> String {
            let word = list.choose(&mut rand::thread_rng()).unwrap();
            println!("word {:?}", word);
            word.to_string()
        }
        //Still don't know if I should keep this function, it's basically for producing alpahbets
        fn produce_alphabets(self) {
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
    let guess_chars = vec![];
    let mut player_one = PlayerRoot::new(
        &random_word,
        0,
        letters,
        list_of_words,
        0,
        6,
        "".to_string(),
        guess_chars,
    );
    loop {
        //Takes in an input
        let mut guess = String::from("");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("You guessed: {}", guess);
        let altered_guess = guess.trim().chars().next().unwrap();
        'outer: for n in random_word.char_indices() {
   
            // if guess.trim().len() > 0 {
            //     println!("yes")
            // }
            if !player_one.correct_guesses.contains(&n.1) {
                // println!("shiit {:?}", player_one.correct_guesses.len());
                if n.1 == altered_guess {
                    player_one.correct_guesses.push(n.1);
                    player_one.no_of_guesses += 1;
                }
            } else {
                //  player_one.no_of_guesses += 1;
                // println!("choose another word");
                continue 'outer;
            }
            println!(
                "correct guess list{:?}, no of guesses {:?}",
                player_one.correct_guesses, player_one.no_of_guesses
            )
        }

        // break;
    }
}
