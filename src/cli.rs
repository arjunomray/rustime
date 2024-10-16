use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long, default_value_t = 25)]
    pub session_time: i32,
    #[arg(short, long, default_value_t = 5)]
    pub break_time: i32,
    #[arg(short('n'), long, default_value_t = String::from("Working on Stuff"))]
    pub session_title: String,
    #[arg[short, long, default_value_t = false]]
    pub minimal_version: bool,
}
