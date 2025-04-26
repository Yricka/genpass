use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use colored::Colorize;

pub fn check_password_strength(password: &str, is_russian: bool) {
    let common_passwords: HashSet<&str> = [
        "123456", "password", "123456789", "qwerty", "12345678", "111111", "123123",
        "abc123", "password1", "1234567", "qwerty123", "000000", "1q2w3e4r"
    ]
    .iter()
    .cloned()
    .collect();

    let sequential_patterns = [
        "abcdefghijklmnopqrstuvwxyz",
        "qwertyuiopasdfghjklzxcvbnm",
        "1234567890",
    ];

    let cleaned_password: String = password.chars()
    .filter(|c| !c.is_control())
    .collect();

    let lower_password = password.to_lowercase();
    let length = cleaned_password.chars().count();
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_special = password.chars().any(|c| !c.is_alphanumeric());

    let unique_chars: HashSet<char> = password.chars().collect();
    let repetition_score = unique_chars.len() as f64 / password.len() as f64;

    let mut charset_size = 0;
    if has_lowercase { charset_size += 26; }
    if has_uppercase { charset_size += 26; }
    if has_digit { charset_size += 10; }
    if has_special { charset_size += 32; }

    let entropy = if charset_size > 0 {
        (password.len() as f64) * (charset_size as f64).log2()
    } else {
        0.0
    };

    let mut issues = Vec::new();
    let mut suggestions = Vec::new();

    if length < 8 {
        issues.push(localize(is_russian, "Пароль слишком короткий (<8 символов)", "Password is too short (<8 characters)"));
        suggestions.push(localize(is_russian, "Увеличьте длину до 12-16 символов", "Increase length to 12-16 characters"));
    }
    if !has_uppercase {
        issues.push(localize(is_russian, "Нет заглавных букв", "No uppercase letters"));
        suggestions.push(localize(is_russian, "Добавьте заглавные буквы", "Add uppercase letters"));
    }
    if !has_lowercase {
        issues.push(localize(is_russian, "Нет строчных букв", "No lowercase letters"));
        suggestions.push(localize(is_russian, "Добавьте строчные буквы", "Add lowercase letters"));
    }
    if !has_digit {
        issues.push(localize(is_russian, "Нет цифр", "No digits"));
        suggestions.push(localize(is_russian, "Добавьте цифры", "Add digits"));
    }
    if !has_special {
        issues.push(localize(is_russian, "Нет специальных символов", "No special characters"));
        suggestions.push(localize(is_russian, "Добавьте спецсимволы (!@#$%)", "Add special characters (!@#$%)"));
    }
    if repetition_score < 0.7 {
        issues.push(localize(is_russian, "Много одинаковых символов", "Too many repeated characters"));
        suggestions.push(localize(is_russian, "Используйте больше разных символов", "Use more diverse characters"));
    }
    if common_passwords.contains(lower_password.as_str()) {
        issues.push(localize(is_russian, "Пароль слишком популярный", "Password is too common"));
        suggestions.push(localize(is_russian, "Выберите более уникальный пароль", "Choose a more unique password"));
    }
    for pattern in sequential_patterns.iter() {
        for window in pattern.as_bytes().windows(4) {
            let seq = String::from_utf8_lossy(window);
            if lower_password.contains(seq.as_ref()) {
                issues.push(localize(is_russian, "Обнаружена последовательность символов", "Sequential characters detected"));
                suggestions.push(localize(is_russian, "Избегайте простых последовательностей (abc, 1234)", "Avoid easy sequences (abc, 1234)"));
                break;
            }
        }
    }

    println!("{}", localize(is_russian, "Анализ пароля:", "Password Analysis:").bright_yellow().bold());
    println!("{}: {}", localize(is_russian, "Длина", "Length"), length);
    println!("{}: {:.2}", localize(is_russian, "Энтропия", "Entropy"), entropy);

    println!("{}", localize(is_russian, "Качество пароля:", "Password quality:").bright_cyan().bold());
    show_strength_bar(entropy);

    if issues.is_empty() {
        println!("{}", localize(is_russian, "✅ Пароль выглядит очень безопасным!", "✅ Password looks very strong!").bright_green().bold());
    } else {
        println!("{}", localize(is_russian, "⚠️ Обнаружены проблемы:", "⚠️ Issues detected:").bright_red().bold());
        for issue in issues {
            println!("- {}", issue.bright_red());
        }
        println!();
        println!("{}", localize(is_russian, "💡 Советы по улучшению:", "💡 Tips for improvement:"));
        for suggestion in suggestions {
            println!("- {}", suggestion.bright_green());
        }
    }
}

pub fn check_passwords_from_file(file_path: &str, is_russian: bool) {
    let file = File::open(file_path).expect("❌ Failed to open file");
    let reader = BufReader::new(file);

    println!("{}", localize(
        is_russian,
        "📄 Проверка паролей из файла:",
        "📄 Checking passwords from file:"
    ).bright_yellow().bold());

    for (i, line) in reader.lines().enumerate() {
        match line {
            Ok(password) => {
                let password = password.trim();
                if !password.is_empty() {
                    println!("\n{} {}", 
                        localize(is_russian, "🔹 Пароль #", "🔹 Password #").bright_cyan(),
                        (i + 1)
                    );
                    check_password_strength(password, is_russian);
                }
            }
            Err(e) => {
                eprintln!("{}: {}", 
                    localize(is_russian, "❌ Ошибка чтения строки", "❌ Failed to read line"),
                    e
                );
            }
        }
    }
}

pub fn localize<'a>(is_russian: bool, ru: &'a str, en: &'a str) -> &'a str {
    if is_russian { ru } else { en }
}

fn show_strength_bar(entropy: f64) {
    let max_entropy = 100.0;
    let percentage = (entropy / max_entropy).min(1.0);

    let filled_blocks = (percentage * 20.0).round() as usize;
    let empty_blocks = 20 - filled_blocks;

    let bar = format!(
        "[{}{}]",
        "█".repeat(filled_blocks).bright_green(),
        "░".repeat(empty_blocks).bright_black()
    );

    println!("{} {:.0}%", bar, percentage * 100.0);
}