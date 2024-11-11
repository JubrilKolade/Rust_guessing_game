use std::{cmp::Ordering, io}; //used for input
//import rand, cargo add rand
use rand::Rng;

fn main() {
    // println!("Hello, world!");
    let correct = rand::thread_rng().gen_range(1..=10);
    println!("correct: {correct}");
    println!("Hi, make a guess?");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("error reading input");

    let guess: u32 = guess.trim().parse().expect("error with parse");


   //CONTROL FLOW 
   //this
    // if correct < guess {
    //     println!("you guessed higher")
    // } else if correct > guess {
    //     println!("you guessed lower")
    // } else {
    //     println!("you guessed right")
    // };

    //or this
    // let mut message = String::new();
    // if correct < guess {
    //     message = String::from("you guessed too high");
    // } else if correct > guess {
    //     message = String::from("you guessed too low");
    // } else {
    //     message = String::from("you guessed just right");
    // }

    // println!("{message}");

    //or this
    // let mut message = if correct < guess {
    //     String::from("you guessed too high")
    // } else if correct > guess {
    //      String::from("you guessed too low")
    // } else {
    //      String::from("you guessed just right")
    // };

    // println!("{message}");

    //or this
    let message = match  guess.cmp(&correct) {
        Ordering::Greater => "you guessed too high",
        Ordering::Less => "you guessed too low",
        Ordering::Equal => "you guessed just right",
    };

    println!("{message}");
}