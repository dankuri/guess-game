use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("guess my num, pls...");

    let secret_num = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("type ur guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("dis not line bruv :/");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("ur guess: {guess}");

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("small."),
            Ordering::Greater => println!("big."),
            Ordering::Equal => {
                println!("enough.");
                break;
            }
        }
    }
}
