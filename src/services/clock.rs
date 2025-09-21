use crate::db::db::SessionInfo;
use crate::services::ascii_art::render_art;
use crate::services::num_to_ascii::get_ascii;
use notify_rust::Notification;
use std::thread;
use std::time::Duration;

pub enum SessionType {
    BREAK,
    SESSION,
}

pub struct Clock<'a> {
    pub session_info: &'a SessionInfo,
    pub session_time: i32,
    pub break_time: i32,
    pub is_minimal: &'a bool,
}

impl<'a> Clock<'a> {
    pub fn new(
        session_info: &'a SessionInfo,
        session_time: i32,
        break_time: i32,
        is_minimal: &'a bool,
    ) -> Self {
        Clock {
            session_info,
            session_time,
            break_time,
            is_minimal,
        }
    }

    fn render_clock(&self, disp_min: i32, disp_sec: i32) {
        let disp_min_tens = (disp_min / 10) % 10;
        let disp_min_ones = disp_min % 10;
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
                "{}  {}      :     {}  {}",
                ascii_min_tens_lines[i],
                ascii_min_ones_lines[i],
                ascii_sec_tens_lines[i],
                ascii_sec_ones_lines[i],
            )
        }
    }

    pub fn countdown(&self, session_type: SessionType, session_counter: i32) {
        let mut sec = match session_type {
            SessionType::SESSION => self.session_time * 60,
            SessionType::BREAK => self.break_time * 60,
        };

        while sec != 0 {
            print!("\x1B[2J\x1B[1;1H"); // used to clear the terminal screen
            let disp_min = sec / 60;
            let disp_sec = sec % 60;

            println!("Currently working on : {}", self.session_info.name);

            self.render_clock(disp_min, disp_sec);
            println!();
            if !self.is_minimal {
                render_art(&session_type);
            }

            println!("{} session(s) so far...", session_counter);
            thread::sleep(Duration::from_secs(1));
            sec -= 1;
        }
        print!("\x1B[2J\x1B[1;1H"); // used to clear the terminal screen
        self.render_clock(00, 00);
        match session_type {
            SessionType::BREAK => {
                let notification = Notification::new()
                    .summary("Break ended")
                    .body("Your break has ended get back to work")
                    .show();
                match notification {
                    Ok(_) => {
                        println!("Send a notification");
                    }
                    Err(e) => {
                        eprintln!("Failed to send a notification: {e}");
                    }
                }
            }
            SessionType::SESSION => {
                let notification = Notification::new()
                    .summary("Session ended")
                    .body("Your session has ended, take a break")
                    .show();

                match notification {
                    Ok(_) => {
                        println!("Send a notification");
                    }
                    Err(e) => {
                        eprintln!("Failed to send a notification: {e}");
                    }
                }
            }
        }
    }
}
