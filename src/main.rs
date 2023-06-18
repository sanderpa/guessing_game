use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // Uncomment the line below for testing purposes
    // println!("Secret number is {}", secret_number);

    loop {
        println!("Input your number: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("\x1B[31mToo small!\x1B[0m"),
            Ordering::Equal => {
                println!("\x1B[32mYou are right!\x1B[0m");
                println!("\u{001b}[33mCongradulations! \u{001b}[0m\u{1F389}");
                break;
            }
            Ordering::Greater => println!("\x1B[31mToo big!\x1B[0m"),
        }
    }
}
