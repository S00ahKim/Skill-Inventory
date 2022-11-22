extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng; //trait

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
    
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() { // shadowing
            Ok(num) => num,
            Err(_) => continue, // expect 대신 이걸 쓰면 종료 대신 처리
        };
    
        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number) { // cmp는 Ordering enum을 리턴
            // match는 arm들로 이루어진다.
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    }
}