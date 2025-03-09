use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long, default_value_t = 25)]
    pub session_time: i32,

    #[arg(short, long, default_value_t = 5)]
    pub break_time: i32,

    #[arg(short('t'), long)]
    pub session_title: Option<String>,

    #[arg[short, long, default_value_t = false]]
    pub minimal_version: bool,

    #[arg(short, long, default_value_t = false)]
    pub list_sessions: bool,

    #[arg(short, long)]
    pub new_session: Option<String>,
}
