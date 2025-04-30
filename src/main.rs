use clap::Parser;
use colored::Colorize;

mod args;
mod gen_pass;
mod check_pass;
mod show_help;
mod show_version;

fn main() {
    let _ = enable_ansi_support::enable_ansi_support();

    let args = args::Args::parse();
    let is_russian = args.ru.unwrap_or(false);

    if args.version.unwrap_or(false) {
        show_version::version(is_russian);
        return;
    }

    if args.help.unwrap_or(false) {
        show_help::help(is_russian);
        return;
    }

    if let Some(password) = &args.check.clone() {
        check_pass::check_password_strength(password, is_russian);
        return;
    } else if let Some(file_path) = &args.check_file.clone() {
        check_pass::check_passwords_from_file(file_path, is_russian);
        return;
    }
    
    for _ in 0..args.count {
        let password = gen_pass::generate_password(
            args.length,
            args.uppercase,
            args.numbers,
            args.special,
        );
        println!("Your password:{}", password.bright_green());
    }

    println!("The generated password was copied to the buffer.");
}