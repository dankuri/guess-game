use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("guess my num (1 to 100), u have 8 tries");

    let secret_num = rand::thread_rng().gen_range(1..=100);
    let mut guess_count = 0;

    while guess_count < 8 {
        println!("type ur guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("dis not line bruv :/");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        guess_count += 1;
        println!("ur guess: {guess} (ur {guess_count} attempt)");

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("small."),
            Ordering::Greater => println!("big."),
            Ordering::Equal => {
                println!("enough on {guess_count} attempt.");
                break;
            }
        }
    }

    if guess_count == 8 {
        println!("u lost. u must be faster than binary search!")
    }
}
