use rand::prelude::{IndexedRandom, SliceRandom};
use arboard::Clipboard;

pub fn generate_password(
    length: usize, 
    use_uppercase: bool, 
    use_numbers: bool,
    use_specials: bool,
) -> String {
    let mut charset = String::from("abcdefghijklmnopqrstuvwxyz");
    let uppercase = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let numbers = "0123456789";
    let specials = "!@#$%^&*()-_=+[]{};:,.<>?/\\|";

    if use_uppercase {
        charset.push_str(uppercase);
    }

    if use_numbers {
        charset.push_str(numbers);
    }

    if use_specials {
        charset.push_str(specials);
    }

    if charset.is_empty() {
        panic!("No available characters for password generation!");
    }

    let mut rng = rand::rng();
    let mut password_chars = Vec::new();

    if use_uppercase {
        password_chars.push(*uppercase.chars().collect::<Vec<_>>().choose(&mut rng).unwrap());
    }

    if use_numbers {
        password_chars.push(*numbers.chars().collect::<Vec<_>>().choose(&mut rng).unwrap());
    }

    if use_specials {
        password_chars.push(*specials.chars().collect::<Vec<_>>().choose(&mut rng).unwrap());
    }

    while password_chars.len() < length {
        password_chars.push(*charset.chars().collect::<Vec<_>>().choose(&mut rng).unwrap());
    }

    password_chars.shuffle(&mut rng);

    let password: String = password_chars.into_iter().collect();

    if let Ok(mut clipboard) = Clipboard::new() {
        let _ = clipboard.set_text(password.clone());
    }

    password
}