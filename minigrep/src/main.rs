use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // 成功時にユニット型を返すため、unwrap_or_else()を使用しない
    if let Err(err) = minigrep::run(config) {
        println!("Application error: {}", err);

        process::exit(1);
    }
}