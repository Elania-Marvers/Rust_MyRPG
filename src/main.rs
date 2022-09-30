/*
** my rpg is a rust program to permit me to know Rust programming langage
** ! is a symbol to precise that we call a macro function
 */


use std::io; // la stdio.h de rust
use std::cmp::Ordering;
use rand::Rng;

fn main ()
{
    println!("Guess the number!");
    println! ("Please input your guess.");

    let secret_number = rand::thread_rng().gen_range(1..=100);



    loop
    {

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => return println!("You win!"),
        }
    }

}

    