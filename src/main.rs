#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
use std::env;
use std::fs;

mod brainfuck;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("[Usage]\n{} [source_file]", args[0]);
        return;
    }

    let source = {
        if let Ok(source) = fs::read_to_string(&args[1]) {
            source
        } else {
            println!("[Error]\nCould not open file");
            return;
        }
    };

    let commands = brainfuck::parse(&source);

    brainfuck::interpret(&commands);
}
