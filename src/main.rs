use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("guess my num (1 to 100), pls...");

    let secret_num = rand::thread_rng().gen_range(1..=100);
    let mut guess_count = 0;

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
        guess_count += 1;

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("small. (ur {guess_count} attempt)"),
            Ordering::Greater => println!("big. (ur {guess_count} attempt)"),
            Ordering::Equal => {
                println!("enough on {guess_count} attempt.");
                break;
            }
        }
    }
}
