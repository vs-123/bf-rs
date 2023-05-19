use std::io::{self, Read, Write};

#[derive(Debug, Clone, Copy)]
pub enum Command {
    Increment(usize),
    Decrement(usize),
    MoveLeft(usize),
    MoveRight(usize),
    LoopOpen(usize),  // usize -> End of loop
    LoopClose(usize), // usize -> Start of loop
    PrintCell,
    InputCell,
}

pub fn parse(source: &str) -> Vec<Command> {
    let mut output = Vec::<Command>::with_capacity(source.len());
    let mut loop_stack = Vec::new();
    let source_bytes = source.as_bytes();

    let mut count = 0;

    for (character_index, character) in source_bytes.iter().enumerate() {
        match character {
            b'+' => {
                if let Some(Command::Increment(count)) = output.last_mut() {
                    *count += 1;
                } else {
                    output.push(Command::Increment(1));
                }
            }

            b'-' => {
                if let Some(Command::Decrement(count)) = output.last_mut() {
                    *count += 1;
                } else {
                    output.push(Command::Decrement(1));
                }
            }

            b'<' => {
                if let Some(Command::MoveLeft(count)) = output.last_mut() {
                    *count += 1;
                } else {
                    output.push(Command::MoveLeft(1));
                }
            }

            b'>' => {
                if let Some(Command::MoveRight(count)) = output.last_mut() {
                    *count += 1;
                } else {
                    output.push(Command::MoveRight(1));
                }
            }

            b'[' => {
                let loop_start = output.len();
                output.push(Command::LoopOpen(character_index));
                loop_stack.push(loop_start);
            }

            b']' => {
                if let Some(loop_start) = loop_stack.pop() {
                    output[loop_start] = Command::LoopOpen(output.len());
                    output.push(Command::LoopClose(loop_start));
                } else {
                    println!("\n[Error at command index {}]", output.len());
                    println!("Unmatched ']'");
                    std::process::exit(1);
                }
            }

            b'.' => output.push(Command::PrintCell),
            b',' => output.push(Command::InputCell),

            // Ignore other characters
            _ => {},
        }
    }

    output
}

pub fn interpret(commands: &[Command]) {
    let mut memory = [0_u8; 30_000];
    let mut pointer = 0_usize;
    let mut command_index = 0;
    let commands_len = commands.len();

    let mut stdout = io::stdout();
    let mut stdin = io::stdin();

    while command_index < commands_len {
        let command = commands[command_index];

        match command {
            Command::Increment(count) => {
                memory[pointer] = memory[pointer].wrapping_add(count as u8);
            }

            Command::Decrement(count) => {
                memory[pointer] = memory[pointer].wrapping_sub(count as u8);
            }

            Command::MoveLeft(count) => {
                pointer = pointer.saturating_sub(count);
            }

            Command::MoveRight(count) => {
                pointer = pointer.saturating_add(count);
            }

            Command::PrintCell => {
                stdout.write(&[memory[pointer]]).and(stdout.flush()).ok();
            }

            Command::InputCell => {
                stdin.read(&mut memory[pointer..=pointer]).ok();
            }

            Command::LoopOpen(loop_end) => {
                if memory[pointer] == 0 {
                    command_index = loop_end;
                }
            }

            Command::LoopClose(loop_start) => {
                if memory[pointer] != 0 {
                    command_index = loop_start;
                }
            }
        }

        command_index += 1;
    }
}