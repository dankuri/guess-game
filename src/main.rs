use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, stdin, stdout, Write};

fn main() {
    let mut guess_mode = String::new();

    print!("u guess or i guess?: ");
    stdout().flush().expect("unable to flush stdout!"); // if not present - print!() will appear after the input

    stdin()
        .read_line(&mut guess_mode)
        .expect("no like dis line bruv :/");

    guess_mode = guess_mode.trim().to_string(); // trimming trailing \n and converting back to String (from &str)

    while guess_mode != "u" && guess_mode != "i" {
        guess_mode = String::new();

        print!("bruv. type u or i: ");
        stdout().flush().expect("unable to flush stdout!");

        stdin()
            .read_line(&mut guess_mode)
            .expect("no like dis line bruv :/");

        guess_mode = guess_mode.trim().to_string();
    }

    if guess_mode == "u" {
        guess_reverse();
        return;
    }

    println!("guess my num (1 to 100), u have 8 attempts");

    let secret_num = rand::thread_rng().gen_range(1..=100);
    let mut guess_count = 0;

    while guess_count < 8 {
        print!("type ur guess: ");
        stdout().flush().expect("unable to flush stdout!");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("no like dis line bruv :/");

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

fn guess_reverse() {
    println!("u chose me to guess..");
    println!("so think about a num from 1 to 100.");
    let mut lower: u32 = 1;
    let mut higher: u32 = 100;
    let mut guess: u32 = rand::thread_rng().gen_range(lower..=higher);
    let mut attempts: u32 = 0;
    println!("is ur num {guess}?");
    println!(
        "for ur answer pls type + if ur num is bigger, - if ur num is smaller and = if i won!"
    );

    let mut answer = String::new();

    io::stdin()
        .read_line(&mut answer)
        .expect("no like dis line bruv :/");

    answer = answer.trim().to_string();

    while answer != "=" {
        attempts += 1;
        match answer.as_str() {
            "+" => {
                lower = guess + 1;
            }
            "-" => {
                higher = guess - 1;
            }
            _ => println!("dunno wat dat means.."),
        }

        let is_valid_answer = check_answer(lower, higher);
        if is_valid_answer {
            guess = rand::thread_rng().gen_range(lower..=higher);
            println!("so ur num is between {lower} and {higher}..")
        } else {
            println!("can't be true, let's try again..");
            lower = 1;
            higher = 100;
            guess = rand::thread_rng().gen_range(lower..=higher);
        }

        answer = String::new();

        print!("is ur num {guess} ({attempts} attempt)? ");
        stdout().flush().expect("unable to flush stdout!");
        io::stdin()
            .read_line(&mut answer)
            .expect("no like dis line bruv :/");
        answer = answer.trim().to_string();
    }

    println!("i won at my {attempts} attempt!");
}

fn check_answer(lower: u32, higher: u32) -> bool {
    if lower >= higher {
        return false;
    } else {
        return true;
    }
}
