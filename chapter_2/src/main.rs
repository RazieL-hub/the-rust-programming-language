use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("guess the number");
    let mut rng = rand::rng();
    let secret_number: u32 = rng.random_range(1..101);
    println!("Secret number is {}", secret_number);
    loop {
        println!("enter your number");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read the line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("your number is {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your number is less"),
            Ordering::Greater => println!("Your number is greated"),
            Ordering::Equal => {
                println!("Great! You win!");
                break;
            }
        }
    }
}