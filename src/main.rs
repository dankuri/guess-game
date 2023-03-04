use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Input, Select};
use rand::Rng;
use std::cmp::Ordering;
use std::{thread, time::Duration};

fn main() {
    let term = Term::stdout();
    term.clear_screen().unwrap();
    term.set_title("dankuri's guess game");
    println!(
        "{}",
        style("welcome to dankuri's guess game!")
            .green()
            .bold()
            .bright()
    );
    thread::sleep(Duration::from_secs_f32(0.5));

    let guess_modes = &["player", "computer"];
    let selected_mode = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick who guesses")
        .default(0)
        .items(guess_modes)
        .interact()
        .unwrap();

    match guess_modes[selected_mode] {
        "computer" => cpu_guess(),
        "player" => player_guess(),
        _ => println!("how you got here?"),
    }
}

fn player_guess() {
    let term = Term::stdout();
    term.set_title("yoo you guessinnnng");
    println!("guess my num (1 to 100), u have 8 attempts");

    let secret_num = rand::thread_rng().gen_range(1..=100);
    let mut guess_count = 0;

    while guess_count < 8 {
        let mut input: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Type in number from 1 to 100")
            .interact_text()
            .unwrap();

        while input.parse::<u32>().is_err() {
            println!("Please type number from 1 to 100!!!");
            input = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("C'mon bruh")
                .interact_text()
                .unwrap();
        }

        let guess: u32 = input.parse().unwrap();
        guess_count += 1;

        println!(
            "ur guess: {} (ur {} attempt)",
            style(guess).bold().cyan(),
            style(guess_count).red()
        );

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("{}", style("small.").red()),
            Ordering::Greater => println!("{}", style("big.").red()),
            Ordering::Equal => {
                println!(
                    "yep. you got {} on {} attempt.",
                    style(secret_num).bold().green(),
                    style(guess_count).bold().green()
                );
                break;
            }
        }
    }

    if guess_count > 8 {
        println!(
            "u {}. u must be faster than binary search!",
            style("lost").bold().red()
        );
    }
}

fn cpu_guess() {
    println!("u chose {} to guess..", style("me").bold().cyan());
    let term = Term::stdout();
    term.set_title("this kamputer is guessiiiiinnnnnnnnnng");

    let half_sec = Duration::from_secs_f32(0.5);
    let two_secs = Duration::from_secs(2);

    thread::sleep(half_sec);
    println!("so think about a num from 1 to 100.");

    let mut lower: u32 = 1;
    let mut higher: u32 = 100;
    let mut guess: u32 = 0;
    let mut attempts: u32 = 0;

    thread::sleep(two_secs);

    while lower != higher {
        attempts += 1;

        thread::sleep(half_sec);
        println!("my {} attempt..", style(attempts).red());

        guess = rand::thread_rng().gen_range(lower..=higher);
        let answers = &["yep", "less.", "more."];
        let answer = Select::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("is ur num {}", style(guess).yellow().bold()))
            .default(0)
            .items(answers)
            .interact()
            .unwrap();

        match answers[answer] {
            "yep" => break,
            "less." => higher = guess - 1,
            "more." => lower = guess + 1,
            _ => println!("yeah you can't be here"),
        }

        match lower.cmp(&higher) {
            Ordering::Equal => break,
            Ordering::Less => {
                thread::sleep(half_sec);
                println!(
                    "so ur num is between {} and {}..",
                    style(lower).cyan(),
                    style(higher).cyan()
                );
            }
            Ordering::Greater => {
                println!("can't be true, let's try again..");
                lower = 1;
                higher = 100;
                attempts = 0;
            }
        }
    }

    if lower == higher {
        guess = lower;
        attempts += 1;
    }

    thread::sleep(half_sec);
    println!(
        "i won at my {} attempt! da num is {}!!!",
        style(attempts).bold().green(),
        style(guess).bold().green()
    );
}
