# GenPass v 1.2

**GenPass** is a powerful tool for generating and validating passwords, written in **Rust**. It supports flexible customization of lengths, character types (lowercase, uppercase, digits, special symbols), and password strength checking.

## 🚀 Features

- Generate random passwords with various options.
- Analyze password strength and detect weaknesses.
- Check multiple passwords from a file.
- Output available in Russian and English.
- High entropy for secure passwords.

## 📋 Installation

To use GenPass on your computer, you need to have Rust installed (if it's not installed yet). You can download and install it from this [link](https://www.rust-lang.org/tools/install).

Then clone the repository:

```bash
git clone https://github.com/Yricka/genpass.git
```

Navigate to the project directory and build it:

```bash
cd genpass
cargo build --release
```

Or use the pre-built version available [here](https://github.com/Yricka/genpass/releases).

## Instructions after installation or compilation:
- Open Settings → System → About → Advanced system settings.
- Click the Environment Variables button.
- Find the Path variable in the list and click Edit.
- Add the path to the folder where you installed the program.
- Open cmd.exe (Command Prompt).
- Enter the command:
```bash
genpass -v
```

## 🛠️ Usage
## Generate a password
To generate a password with default settings (16 characters, using all character types):
```bash
genpass
```

## Options for password generation

- -l, --length <N> — password length (default: 16)
- -c, --count <N> — number of passwords to generate (default: 1)
- --uppercase — disable uppercase letters (enabled by default)
- --numbers — disable digits (enabled by default)
- --special — disable special characters (enabled by default)
- -h, --help — show application help.

Example:
```bash
genpass -l 12 -c 3 --upercase
```

## To check password strength
```bash
genpass --check <PASSWORD>
```

Example:
```bash
genpass --check "MySecurePassword123!"
```

## Checking passwords from a file
If you have a list of passwords in a file, such as passwords.txt, you can check them using:
```bash 
genpass --check-file passwords.txt
```

## Example of password analysis output
```bash 
Password Analysis:
🧮: Length: 16
🔐: 104.87 / 104.87 (100%)
Password class: 🟢 Strong
[████████████████████] 100%
✅ Password looks very strong!
```

## 🌍 Localization
You can choose the output language using the -r or --ru flag for Russian (English is the default).

Example:
```bash
genpass --check "Пароль123!" -r
```

## 💡 Tips for improving your password
- Increase the password length to 12–16 characters.
- Use both uppercase and lowercase letters.
- Add special characters (!@#$%).
- Include digits for better security.
- Avoid using common or simple passwords.

## Author
- whynotq