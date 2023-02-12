use crossterm::{
    style::{self, Color, Stylize},
    terminal::{Clear, ClearType},
};
use std::io::{stdout, Write};
use std::time::Duration;

const DELAY: Duration = Duration::from_secs(1);

fn get_time() -> String {
    let now = chrono::Local::now();
    let hours = now.format("%H").to_string();
    let minutes = now.format("%M").to_string();
    let seconds = now.format("%S").to_string();

    format!(
        "{}:{}:{}",
        style::style(hours).with(Color::Yellow),
        style::style(minutes).with(Color::Green),
        style::style(seconds).with(Color::Red),
    )
}

fn render() {
    print!("{}", Clear(ClearType::All));

    let mut stdout = stdout();
    println!("{}", get_time());
    stdout.flush().unwrap();
}

fn main() {
    loop {
        render();
        std::thread::sleep(DELAY);
    }
}
