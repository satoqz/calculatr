mod calc;

use crate::calc::{add, subtract};
use clap::{value_parser, Arg, Command};
use colored::Colorize;
use spinners::{Spinner, Spinners};
use std::{thread, time::Duration};

fn main() {
    let matches = Command::new("calculatr")
        .arg(
            Arg::new("first number")
                .required(true)
                .value_parser(value_parser!(i32)),
        )
        .arg(Arg::new("operator").required(true).value_parser(["+", "-"]))
        .arg(
            Arg::new("second number")
                .required(true)
                .value_parser(value_parser!(i32)),
        )
        .get_matches();

    println!(
        "Welcome to {}, the next-gen blazing fast calculator! 🚀 🦄",
        "Calculatr".purple().bold().underline().italic()
    );

    let op: &String = matches.get_one("operator").unwrap();
    let x: i32 = *matches.get_one("first number").unwrap();
    let y: i32 = *matches.get_one("second number").unwrap();

    println!(
        "{} {} {} {}",
        "Task:".italic(),
        x.to_string().yellow().bold(),
        op.green(),
        y.to_string().yellow().bold()
    );

    let mut spinner = Spinner::new(Spinners::Aesthetic, "Calculating...".into());
    thread::sleep(Duration::from_secs(2));
    spinner.stop();

    let mut spinner = Spinner::new(Spinners::Aesthetic, "This may take a while...".into());
    thread::sleep(Duration::from_secs(2));
    spinner.stop_with_newline();

    let result = match op.as_str() {
        "+" => add(x, y),
        "-" => subtract(x, y),
        _ => unreachable!(),
    };

    println!(
        "\n{} {}\n",
        "Result:".italic(),
        result.to_string().red().bold()
    );
}
