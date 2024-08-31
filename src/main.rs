use clap::Parser;
use notify_rust::Notification;
use std::process;
use std::time::Duration;
use std::{io, thread};

mod num_to_ascii;

use num_to_ascii::get_ascii;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = 25)]
    session_time: i32,
    #[arg(short, long, default_value_t = 5)]
    break_time: i32,
}

enum SessionType {
    BREAK,
    SESSION,
}

fn countdown(min: i32, sesssion_type: SessionType) {
    let mut sec = min * 60;

    while sec != 0 {
        print!("\x1B[2J\x1B[1;1H"); // used to clear the terminal screen
        let disp_min = sec / 60;
        let disp_min_tens = (disp_min / 10) % 10;
        let disp_min_ones = disp_min % 10;
        let disp_sec = sec % 60;
        let disp_sec_tens = (disp_sec / 10) % 10;
        let disp_sec_ones = disp_sec % 10;

        let ascii_min_tens = get_ascii(disp_min_tens);
        let ascii_min_ones = get_ascii(disp_min_ones);
        let ascii_sec_tens = get_ascii(disp_sec_tens);
        let ascii_sec_ones = get_ascii(disp_sec_ones);

        let ascii_min_tens_lines: Vec<&str> = ascii_min_tens.lines().collect();
        let ascii_min_ones_lines: Vec<&str> = ascii_min_ones.lines().collect();
        let ascii_sec_tens_lines: Vec<&str> = ascii_sec_tens.lines().collect();
        let ascii_sec_ones_lines: Vec<&str> = ascii_sec_ones.lines().collect();

        println!();
        for i in 1..ascii_min_tens_lines.len() {
            println!(
                "                                                                            {}  {}      :     {}  {}",
                ascii_min_tens_lines[i],
                ascii_min_ones_lines[i],
                ascii_sec_tens_lines[i],
                ascii_sec_ones_lines[i],
            )
        }
        render_cat(&sesssion_type);
        thread::sleep(Duration::from_secs(1));
        sec -= 1;
    }
    print!("\x1B[2J\x1B[1;1H"); // used to clear the terminal screen
    println!("00:00");
    match sesssion_type {
        SessionType::BREAK => {
            Notification::new()
                .summary("Break ended")
                .body("Your break has ended get back to work")
                .show()
                .unwrap();
        }
        SessionType::SESSION => {
            Notification::new()
                .summary("Session ended")
                .body("Your session has ended, take a break")
                .show()
                .unwrap();
        }
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
                    countdown(session_time, SessionType::SESSION);
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
                    countdown(break_time, SessionType::BREAK);
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

fn render_cat(session_type: &SessionType) {
    match session_type {
        SessionType::SESSION => {
            println!(
                r#"

                                                                             ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                                                            ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣾⣿⣇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                                                            ⢰⣤⣤⣄⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣾⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                                                            ⠀⠻⣿⣿⣿⣿⣷⡖⠤⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡎⠻⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                                                            ⠀⠀⠘⢿⣿⣿⣿⡇⠀⠀⠉⠒⢤⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣸⠀⠀⠈⢻⡿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                                                            ⠀⠀⠀⠀⠉⠻⣿⣧⣀⠀⠀⠀⠀⠈⠑⠦⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠇⠀⠀⠀⠀⡅⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                                                            ⠀⠀⠀⠀⠀⠀⠈⠛⢆⡉⠢⢄⡀⠀⠀⠀⠈⠑⢄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡸⠀⠀⠀⠀⢠⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                                                            ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠲⢄⡈⠒⠄⡀⠀⠀⠀⠹⠴⠒⠒⠒⠒⠒⠦⠤⢀⡀⠀⠀⠀⠀⡇⠀⠀⠀⢀⣾⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                                                            ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠑⠦⠌⣳⠖⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠑⠲⣄⠀⡇⠀⠀⠀⡜⡌⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                                                            ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡰⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⢧⡀⠀⡼⢰⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                                                            ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡰⠁⠀⣠⣤⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⢼⢠⠇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                                                            ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⠃⠀⢸⣿⣯⣨⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                                                            ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⠓⢦⠀⠈⠛⠛⠛⠁⠀⠀⢀⣀⠀⠀⠀⠀⠀⣠⣤⣤⡀⠀⠀⢸⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                                                            ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⡁⠀⠘⠀⠀⠀⠀⠀⡀⠀⢀⣀⡉⠁⠀⠀⠀⠀⣿⣤⣾⣿⠀⠀⡼⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                                                            ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠁⠀⣸⠀⠀⠀⠀⠀⣿⠭⣑⠻⢿⣦⢄⣀⠀⠀⠛⠛⠛⠁⠀⢠⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                                                            ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⡤⠞⠁⠀⠀⠀⠀⠀⡯⠉⠀⠈⠙⢾⣿⠏⠀⠀⠀⠀⠀⣀⣀⣸⠀⢀⣀⡀⠤⠤⢤⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                                                            ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠹⡄⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⣠⠋⠀⠀⠀⠀⢀⠞⠁⠀⡿⠊⠉⠀⠀⠀⠀⠸⢱⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                                                            ⠀⠀⠀⠀⠀⠀⠀⠀⣀⡠⠤⠄⠠⠿⠤⠄⣀⡀⠀⠀⠰⡀⢀⣠⠞⠁⠀⠀⠀⠀⠀⣏⠀⠀⣠⠃⠀⠀⠀⠀⠀⢀⣀⡼⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                                                            ⠀⠀⠀⠀⠀⠀⢀⠖⠁⠀⠀⠀⠀⠀⠀⠀⠀⠉⠲⢄⡀⠉⠉⠀⠀⠀⠀⠀⠀⠀⠀⠧⣤⠖⠁⠀⠀⠀⠀⠀⢀⡞⠛⠓⠒⠒⠒⠒⠤⠤⠄⢀⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                                                            ⠀⠀⠀⠀⠀⠀⢸⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢳⡖⠢⠄⣀⠀⠀⠀⣀⡠⠔⠊⠁⠀⠀⠀⠀⢀⡠⣶⠎⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠉⠒⠲⠤⣀⡀⠀⠀⠀
                                                                            ⠀⠀⠀⠀⠀⠀⠈⣦⠤⠤⠄⣀⣀⠀⠀⠀⠀⠀⡀⢠⡾⠉⠒⠊⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡤⠒⣡⠞⡇⠀⡠⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠑⠢⣤
                                                                            ⠀⠀⠀⠀⠀⠀⠀⠈⠓⠤⣀⣀⣀⣀⣠⡤⠤⠚⠋⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡤⠚⠁⣠⠞⠁⢸⠁⢰⠁⠀⠉⠒⢄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⠞⠁
                                                                            ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣞⣠⠔⠋⠀⠀⠀⡌⠀⡆⠀⠀⠀⠀⠀⠈⠑⠠⣀⠀⠀⠀⠀⠀⠀⠀⠀⣠⠞⠁⠀⠀
                                                                            ⠀⠀⠀⠀⠀⠀⠀⠀⠀⡼⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⢀⡀⠀⠀⢰⠃⢰⠁⠀⢠⠖⠒⠤⢄⣀⠀⠈⠑⠢⡀⠀⠀⠀⣠⠞⠁⠀⠀⠀⠀
                                                                            ⠀⠀⠀⠀⠀⠀⠀⠀⢠⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⡛⠭⡉⠉⠀⡈⠀⡰⠃⠀⠀⠀⠀⠀⠉⠁⠒⠦⠬⣳⣠⠞⠁⠀⠀⠀⠀⠀⠀
                                                                            ⠀⠀⠀⠀⠀⠀⠀⢀⠏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⡇⠀⠈⡓⠄⠇⡴⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                                                            ⠀⠀⠀⠀⠀⠀⠀⡞⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣧⣀⣿⡷⣷⡏⠉⠚⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                                                            ⠀⠀⠀⠀⠀⠀⣸⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠹⣠⠉⠀⢸⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                                                            ⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢻⠉⠑⠚⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                                                            ⠀⠀⠀⠀⠀⠀⢧⢠⠒⠂⠤⣄⣀⠀⠀⠀⠀⠀⣀⣀⠀⠀⠀⠀⠀⠀⠠⠄⣀⡀⠀⠀⠀⠀⠆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                                                            ⠀⠀⠀⠀⠀⠀⣀⡷⢄⡀⠀⠀⠀⠀⠀⠀⢀⣀⣀⣀⣀⣀⡀⠀⠀⠀⠀⠀⠀⠈⠑⢤⠀⡼⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                                                            ⠀⠀⠀⢠⣶⣿⡯⢄⠴⢋⣀⣈⡭⠟⠉⠁⠀⠀⠀⠀⠈⠉⠓⠒⠂⠤⠤⢤⣠⡤⠤⠔⠛⠶⢤⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                                                            ⠀⠀⠀⠀⠉⠉⠉⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠓⠒⠂⠤⠤⣈⣙⠻⣷⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                                                            ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠉⠉⠉"#
            );
        }
        SessionType::BREAK => {
            println!(
                r#"
                                                                ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣤⣤⣤⣄⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣤⣬⣿⡿⠿⢿⣶⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                                                ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⡿⠁⠀⠉⠙⠻⠿⣶⣦⣀⣠⣤⣤⣤⣤⣤⣤⣶⠿⠟⠫⠉⠀⠀⢀⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                                                ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⡇⠀⠀⠀⠀⣀⣠⣤⠤⣤⣤⣀⡒⠀⣀⣤⡤⠶⠶⠤⣄⡀⠤⣀⣸⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                                                ⠀⠀⠀⠀⠀⠀⢀⣀⢀⣀⡀⠀⠀⠀⠀⠀⠀⠀⢸⡇⠀⣠⡼⠛⠉⠀⠀⠀⠀⠀⠈⠙⠟⠉⠀⠀⠀⠀⠀⠈⠙⠳⣏⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                                                ⠀⠀⠀⠀⣴⣤⣿⣻⣿⣿⣷⣿⣧⠀⠀⠀⠀⠀⢸⣿⣸⠋⠀⠠⠴⠶⠒⠒⠲⠆⠀⠀⠀⠀⠘⠓⠒⠒⠲⠶⠀⠀⠈⣿⣿⣿⣇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                                                ⠀⠀⠀⢴⡿⣿⡿⠋⠉⠩⠙⣿⣿⣄⠀⠀⠀⠀⣼⣿⠇⠀⠀⠀⠀⠀⠀⣠⣿⣄⣀⣀⣀⣀⣀⣴⣄⠀⠀⠀⠀⠀⠀⡇⢻⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                                                ⠀⠀⠀⠈⣿⡿⠁⠀⠀⠀⠀⠹⣿⣿⣷⣄⠀⢰⣿⣿⠀⠀⠀⠀⠀⢀⣀⣨⣤⣤⠤⠤⠤⠤⠤⣬⣭⣄⣀⣀⠀⢀⡼⠁⠀⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                                                ⠀⠀⠀⢠⣿⠁⠀⠀⠀⠀⠀⠀⠹⣿⣿⣿⣷⣼⣿⣿⣠⠴⠖⠚⠉⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠉⠙⠓⠶⢤⣸⣿⣇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                                                ⠀⠀⠀⠈⣿⠀⠀⠀⠀⠀⠀⠀⠀⠘⢿⣿⣿⡿⠟⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠛⢿⣦⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                                                ⠀⠀⠀⠸⣿⢰⠀⠀⠀⠀⠀⠀⠀⣠⣾⡿⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⡿⣿⣷⣤⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                                                ⠀⠀⠀⠀⣿⡼⡀⠀⠀⠀⠀⣠⣾⡿⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠈⢿⣻⣿⣷⣦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                                                ⠀⠀⠀⠀⢻⣧⢅⠀⠀⢀⡾⣻⠟⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⠇⠀⠘⡇⠹⣇⠙⢿⣦⣀⠀⠀⠀⠀⠀⠀⠀
                                                                ⠀⠀⠀⠀⠘⣿⡌⡀⣰⡟⢡⡏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡜⠀⠀⠀⡗⠀⠹⣆⠀⠙⢿⣷⡀⠀⠀⠀⠀⠀
                                                                ⠀⠀⠀⠀⠀⢻⣧⣰⡟⠀⢸⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠞⠁⠀⠀⣰⠿⠀⠀⢻⡄⠀⠀⠙⢿⣧⠀⠀⠀⠀
                                                                ⠀⠀⠀⠀⠀⠈⣿⡟⡄⠀⠈⢷⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⠔⠃⠀⠀⢀⣴⠏⠂⠀⠀⠘⣧⡀⠀⠀⠀⠻⣷⡀⠀⠀
                                                                ⠀⠀⠀⠀⠀⢰⣿⢹⠀⠀⠀⠈⠳⣄⡀⠀⠀⠀⠀⣀⣠⣤⡴⠶⠒⠒⠛⠛⠛⠛⠛⠛⢛⠛⠒⠒⠒⠒⠶⠶⠶⠶⠿⠥⠤⠤⠴⠶⡟⠁⠀⠀⠀⢀⠀⣿⠀⠀⠀⠀⠀⠹⣷⡀⠀
                                                                ⠀⠀⠀⠀⠀⣼⡏⠀⢠⣄⠀⠀⠀⠀⠉⠙⠛⠉⠉⠙⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⠀⠀⢀⣠⠾⢻⡇⢻⢀⠀⠀⠀⠀⠀⢻⣇⠀
                                                                ⠀⠀⠀⠀⠀⣿⡇⠀⣾⠉⠷⣄⣂⣂⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣤⢴⡟⠁⠀⣼⣧⣼⡧⠄⢀⣠⣤⠀⣿⣿⣦
                                                                ⠀⢀⣄⣀⣀⣿⣧⡶⢿⣀⣠⡼⠋⠉⠉⠙⠳⢦⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⠞⠉⠀⠈⠷⣤⠾⠛⠀⢨⡽⠟⠋⣩⣯⣾⣿⣙⣿
                                                                ⠀⠸⣿⡉⠉⠛⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠹⣿⣦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣰⠟⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⣧⣠⣶⣿⢿⣿⣯⣿⡏
                                                                ⠀⠀⢹⣷⣀⡴⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠸⣿⣿⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢰⠏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠁⠹⣿⣿⣿⠛⠁
                                                                ⠀⢠⣿⠁⠀⠀⠀⠀⠀⢀⡴⠚⠉⢉⣉⣉⠉⠉⠒⠲⣿⣿⣿⣎⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⠀⢀⡠⣖⣦⣭⣤⣥⣐⠒⠦⢄⡀⠀⠀⠀⡤⠿⣿⣦⣤⠀
                                                                ⣤⣾⣇⣀⡀⠀⠀⠀⣠⠎⣠⡴⠛⠉⠉⠉⠹⣆⠀⢀⣿⣿⣿⣿⡄⠀⠀⠀⢀⣀⣀⣀⣀⣀⣀⣀⣀⣀⡀⠀⠀⠀⣿⢀⠎⣾⠋⠀⠀⠀⠀⠉⠳⣄⠀⠙⡄⠀⠀⣧⣀⣤⡾⠟⠀
                                                                ⢿⣥⣀⣺⠇⠀⠀⢠⠇⣰⠏⠀⠀⠀⠀⠀⠀⣿⠀⣸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣶⣿⡜⠀⣇⠀⠀⠀⠀⠀⠀⠀⢹⠀⠀⠁⠀⠀⢸⣿⠁⠀⠀⠀
                                                                ⠀⠹⣿⡁⠀⠀⠀⢸⠀⣯⠀⠀⠀⠀⠀⢀⣴⠃⣰⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⡀⠙⢦⡀⠀⠀⠀⣀⣴⠏⠀⢀⡇⠀⣰⡿⠁⠀⠀⠀⠀
                                                                ⠀⠀⠈⢿⣦⡀⠀⠀⢣⡘⢦⣄⣀⣤⠶⠋⣡⣾⠟⠿⠿⠿⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠿⠿⠿⠿⠟⢿⣦⣀⠉⠉⠉⠉⠉⠀⢀⡤⢋⣠⡾⠋⠀⠀⠀⠀⠀⠀
                                                                ⠀⠀⠀⠀⠉⡛⣷⣶⣤⡭⠷⠤⣤⣴⣶⣟⡛⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⣻⣿⣷⣶⣶⣶⣴⣷⣾⣿⠛⠀⠀⠀⠀⠀⠀⠀⠀
                "#
            );
        }
    }
}

fn main() {
    let args = Args::parse();
    pomodoro(args.session_time, args.break_time);
}
