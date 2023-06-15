use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::process::Command;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..100);

    // println!("your secret_numb is: {secret_number}");

    loop {
        println!("Please input your gusess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please input a number");
                continue;
            },
        };

        println!("your guess: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
            Ordering::Greater => println!("too big"),
        }
    }

    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
}
