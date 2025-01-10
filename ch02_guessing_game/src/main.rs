use std::{cmp::Ordering, io};
use rand::Rng;
use colored::Colorize;

fn main() {

  let secret_number = rand::thread_rng()
  .gen_range(1..=100);

  println!("secret number: {secret_number}");

  loop {
    println!("enter a number: ");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
    .expect("could not read what you entered");

    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };

    println!("you guessed: {guess}");

    match guess.cmp(&secret_number) {
      Ordering::Less => println!("{}","your guess was low".red()),
      Ordering::Greater => println!("{}","your guess was high".red()),
      Ordering::Equal => {
        println!("{}", "DISCO".green().bold());
        break;
      }

    }

  }

}




