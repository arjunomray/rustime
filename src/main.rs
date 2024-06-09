use clap::Parser;
use std::process;
use std::time::Duration;
use std::{io, thread};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = 25)]
    session_time: i32,
    #[arg(short, long, default_value_t = 5)]
    break_time: i32,
}

fn countdown(min: i32) {
    let mut sec = min * 60;

    while sec != 0 {
        print!("\x1B[2J\x1B[1;1H"); // used to clear the terminal screen
        let disp_min = sec / 60;
        let disp_sec = sec % 60;
        let display_time = format!("{:02}:{:02}", disp_min, disp_sec);
        println!("{}", display_time);
        thread::sleep(Duration::from_secs(1));
        sec -= 1;
    }
}

fn pomodoro(session_time: i32, break_time: i32) {
    let mut session_counter = 0;
    loop {
        loop {
            println!("Start session? (Y/n)");
            let mut session_check = String::new();
            io::stdin()
                .read_line(&mut session_check)
                .expect("Failed to read");

            let session_check = session_check.trim().to_lowercase();

            match session_check.as_str() {
                "y" | "yes" => {
                    countdown(session_time);
                    session_counter += 1;
                    break;
                }

                "n" | "no" => {
                    println!("Exiting program... worked for {session_counter} sessions");
                    process::exit(0)
                }
                _ => {
                    println!("Invalid input.");
                    continue;
                }
            }
        }

        loop {
            println!("Start break? (Y/n)");
            let mut break_check = String::new();
            io::stdin()
                .read_line(&mut break_check)
                .expect("Failed to read");

            let break_check = break_check.trim().to_lowercase();

            match break_check.as_str() {
                "y" | "yes" => {
                    countdown(break_time);
                    break;
                }

                "n" | "no" => {
                    println!("Exiting program... worked for {session_counter} sessions");
                    process::exit(0)
                }
                _ => {
                    println!("Invalid input.");

                    continue;
                }
            }
        }
    }
}

fn main() {
    let args = Args::parse();
    pomodoro(args.session_time, args.break_time);
}
