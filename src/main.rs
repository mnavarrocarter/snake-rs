mod point;
mod direction;
mod command;
mod snake;
mod game;
mod ui;
mod consoleui;

use crate::game::Game;
use crate::consoleui::ConsoleUI;
use std::io::stdout;
use clap::Parser;

fn main() {
    let conf = Config::parse();
    Game::new(conf.width, conf.height).run(&mut ConsoleUI::new(stdout()));
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Config {
    #[arg(long, default_value_t = 10)]
    pub width: u16,
    #[arg(long, default_value_t = 10)]
    pub height: u16,
}