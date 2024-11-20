use std::env;
use std::process;
use dont_use_this_grep::run;
use dont_use_this_grep::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        eprintln!("{e}");
        process::exit(1);
    }
}
