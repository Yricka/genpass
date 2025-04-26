use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "whynotq",
    disable_help_flag = true,
    disable_version_flag = true,
)]
pub struct Args {
    #[clap(short, long, default_value_t = 16)]
    pub length: usize,

    #[clap(short, long, default_value_t = 1)]
    pub count: usize,

    #[clap(long, action = clap::ArgAction::SetFalse, default_value_t = true)]
    pub uppercase: bool,

    #[clap(long, action = clap::ArgAction::SetFalse, default_value_t = true)]
    pub numbers: bool,
    
    #[clap(long, action = clap::ArgAction::SetFalse, default_value_t = true)]
    pub special: bool,

    #[clap(short = 'h', long = "help", action = clap::ArgAction::SetTrue)]
    pub help: Option<bool>,

    #[clap(long = "check")]
    pub check: Option<String>,

    #[clap(long = "check-file")]
    pub check_file: Option<String>,

    #[clap(short = 'v', long = "version", action = clap::ArgAction::SetTrue)]
    pub version: Option<bool>,

    #[clap(short = 'r', long = "ru", action = clap::ArgAction::SetTrue)]
    pub ru: Option<bool>,
}