use rand::Rng;

pub fn generate_password(length: usize, use_uppercase: bool, 
    use_numbers: bool, use_specials: bool) -> String {
    let mut charset = String::from("abcdefghijklmnopqrstuvwxyz");

    if use_uppercase {
        charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
    if use_numbers {
        charset.push_str("0123456789");
    }
    if use_specials {
        charset.push_str("!@#$%^&*()-_=+[]{};:,.<>?/\\|");
    }

    if charset.is_empty() {
        panic!("Нет доступных символов для генерации пароля!");
    }

    let mut rng = rand::rng();
    (0..length)
        .map(|_| {
            let idx = rng.random_range(0..charset.len());
            charset.chars().nth(idx).unwrap()
        })
        .collect()

}