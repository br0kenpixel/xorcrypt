use clap::Parser;
use rpassword::read_password;
use std::io::{stdout, Write};
use std::path::PathBuf;
use std::process::exit;
mod crypto;

#[derive(Debug, Parser, Clone)]
#[command(author, version, about)]
struct Options {
    /// Optional name to operate on
    target: PathBuf,

    /// Decrypt mode
    #[arg(short, long, default_value_t = false)]
    reverse: bool,

    /// Password
    #[arg(short, long)]
    password: Option<String>,

    /// Output file. By default the input file is overwritten
    #[arg(short)]
    output: Option<PathBuf>,
}

fn main() {
    let args = Options::parse();

    if !args.target.exists() {
        eprintln!("{}: no such file", args.target.display());
        exit(1);
    }

    let password = args.password.clone().unwrap_or({
        let mut stdlock = stdout().lock();
        stdlock.write_all(b"Password: ").unwrap();
        stdlock.flush().unwrap();
        read_password().unwrap()
    });
    let output = args.output.unwrap_or(args.target.clone());

    if password.is_empty() {
        eprintln!("Password cannot be empty");
        exit(1);
    }

    if args.reverse {
        crypto::decrypt_file(&args.target, output, &password);
    } else {
        crypto::encrypt_file(&args.target, output, &password);
    }
}
