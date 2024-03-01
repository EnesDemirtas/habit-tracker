use core::panic;
use std::fs;
use std::process;

use chrono::Local;
use chrono::LocalResult;
use chrono::TimeZone;
use clap::{arg, ArgAction, Command};

fn main() {
    let matches = Command::new("habit-tracker")
        .args([
            arg!(-i --info "show info about your progress").action(ArgAction::SetTrue),
            arg!(-r --relapse "reset your progress").action(ArgAction::SetTrue),
            arg!(-n --new "start new progress").action(ArgAction::SetTrue),
        ])
        .get_matches();

    let info = matches.get_flag("info");
    if info {
        println!("info command has been called.");

        let last_time_str = fs::read_to_string("my_progress.txt").unwrap_or_else(|err| {
            println!("{err}");
            process::exit(1);
        });

        match Local.timestamp_millis_opt(last_time_str.parse::<i64>().unwrap()) {
            LocalResult::Single(dt) => println!("{:?}", dt),
            _ => panic!("Incorrect timestamp millis"),
        }
    }

    let relapse = matches.get_flag("relapse");
    if relapse {
        println!("relapse command has been called.");
        let now = Local::now().timestamp_millis();
        match fs::write("my_progress.txt", now.to_string()) {
            Ok(()) => println!("Successfully wrote current time to file"),
            Err(_) => panic!("Something went wrong while writing to file"),
        }
    }

    let new = matches.get_flag("new");
    if new {
        let now = Local::now().timestamp_millis();
        let result = fs::write("my_progress.txt", now.to_string());
        match result {
            Ok(()) => println!("Successfully wrote current time to file"),
            Err(_) => panic!("Something went wrong while writing to file"),
        }
    }
}
