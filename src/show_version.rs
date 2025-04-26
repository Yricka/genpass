use figleter::FIGfont;
use colored::Colorize;

use crate::args;
use clap::CommandFactory;

pub fn version(is_russian: bool) {
    let command = args::Args::command();

    let version = command.get_version().unwrap_or("1.0");
    let author = command.get_author().unwrap_or("whynotq");

    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("GENPASS");
    assert!(figure.is_some());

    println!("{}", figure.unwrap());

    if is_russian {
        println!("{}", format!("Версия: {}", version).bright_yellow().bold());
        println!("{}", format!("Автор: {}", author).bright_cyan().bold());
    } else {
        println!("{}", format!("version: {}", version).bright_yellow().bold());
        println!("{}", format!("author: {}", author).bright_cyan().bold());
    }
}