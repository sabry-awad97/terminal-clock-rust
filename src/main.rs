use chrono::Local;
use crossterm::{
    style::{self, Color, Stylize},
    terminal::{Clear, ClearType},
};
use std::{thread, time};

const DELAY: time::Duration = time::Duration::from_secs(1);

fn get_time() -> String {
    let now = Local::now();
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
    println!("{}", get_time());
}

fn main() {
    loop {
        render();
        thread::sleep(DELAY);
    }
}
