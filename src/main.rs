use rand::seq::SliceRandom;
use clap::{Command, Arg};
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
  }


  struct PlayerGuesser {
guess: char,
tries: i8,
  }

impl PlayerRoot {

    fn new(word: &str, no_of_guesses: i8, available_alphabets: Vec<char>,list_of_words_to_guess_from: Vec<String>, _score: i8, max: i8, guess:String )  -> PlayerRoot {
PlayerRoot{word: String::from(word), no_of_guesses,  available_alphabets, list_of_words_to_guess_from, score: 0, max_tries: max, guess}
    }

    fn generate_random_word(list: &Vec<String>) -> String {
let word = list.choose(&mut rand::thread_rng()).unwrap();
println!("word {:?}",word);
word.to_string()


    }
//Still don't know if I should keep this function, it's basically for producing alpahbets
    fn produce_alphabets (self) {
     let letters= String::from("A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y,Z");
for item in letters.split(",") {
    println!("{:?}", item)
}
    }

}
//list of words for the game
let list_of_words = vec!["hunting".to_string(), "dizzy".to_string(), "while".to_string(), "string".to_string(), "something".to_string(), "notified".to_string()];
let random_word = PlayerRoot::generate_random_word(&list_of_words);
 let letters= vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y','Z'];
let player_one = PlayerRoot::new(&random_word, 6, letters,list_of_words, 0, 0, "".to_string());
  loop {
//Takes in an input
            let mut guess = String::from("");
io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("You guessed: {}", guess);
    let altered_guess = guess.chars().next().unwrap();
    for n in random_word.char_indices(){
        println!("{:?}", n.1);
         if n.1 == altered_guess {
            println!("your guess{:?}, the rest {:?}", altered_guess, random_word)
        }     
    }

//     if random_word.contains(altered_guess) {
// println!("{:?}", random_word)
//     }
break;
  }



  

}
