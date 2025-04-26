use figleter::FIGfont;
use colored::Colorize;

pub fn help(is_russian: bool) {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("HELP");
    assert!(figure.is_some());

    println!("{}", figure.unwrap());

    if is_russian {
        println!("{}", "Использование: genpass [ОПЦИИ]".bright_yellow());
        println!();
        println!("{}", "Доступные опции:".bright_cyan());
        println!("{}", "  -l, --length <N>   📏 Длина пароля (по умолчанию 16)");
        println!("{}", "  -c, --count <N>    🔢 Количество паролей (по умолчанию 1)");
        println!("{}", "  --uppercase        🔠 Включить заглавные буквы (по умолчанию ВКЛ)");
        println!("{}", "  --numbers          0️⃣ Включить цифры (по умолчанию ВКЛ)");
        println!("{}", "  --special          💥 Включить специальные символы (по умолчанию ВКЛ)");
        println!("{}", "  --check <PASSWORD> 🔍 Проверить существующий пароль на надежность");
        println!("{}", "  --check-file <pass.txt> 🔍 Проверить несколько паролей в файле");
        println!("{}", "  -v, --version      ℹ️ Показать версию программы");
        println!("{}", "  -h, --help         🆘 Показать это сообщение помощи");
        println!("{}", "  -r, --ru           Перевести интерфейс на русский");

    } else {
        println!("{}", "Usage: genpass [OPTIONS]".bright_yellow());
        println!();
        println!("{}", "Available options:".bright_cyan());
        println!("{}", "  -l, --length <N>   📏 Set password length (default 16)");
        println!("{}", "  -c, --count <N>    🔢 Number of passwords (default 1)");
        println!("{}", "  --uppercase        🔠 Enable uppercase letters (default: ON)");
        println!("{}", "  --numbers          0️⃣ Enable numbers (default: ON)");
        println!("{}", "  --special          💥 Enable special characters (default: ON)");
        println!("{}", "  --check <PASSWORD> 🔍 Check the strength of the existing password");
        println!("{}", "  --check-file <pass.txt> 🔍 Checking multiple passwords in a single file");
        println!("{}", "  -v, --version      ℹ️ Show program version");
        println!("{}", "  -h, --help         🆘 Show this help message");
        println!("{}", "  -r, --ru           Switch interface language to Russian");
    }
}