use std::io::{self, stdout, Read, Write};

#[derive(Debug, Clone, Copy)]
pub enum Command {
    Increment,
    Decrement,
    MoveLeft,
    MoveRight,
    LoopOpen(usize),  // usize -> End of loop
    LoopClose(usize), // usize -> Start of loop
    PrintCell,
    InputCell,
}

pub fn parse(source: &str) -> Vec<Command> {
    let mut output = Vec::<Command>::with_capacity(source.len());
    let mut loop_stack = Vec::new();
    let source_bytes = source.as_bytes();
    let source_bytes_length = source_bytes.len();

    for character_index in 0..source_bytes_length {
        match source_bytes[character_index] {
            b'+' => output.push(Command::Increment),
            b'-' => output.push(Command::Decrement),
            b'<' => output.push(Command::MoveLeft),
            b'>' => output.push(Command::MoveRight),

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
            _ => {}
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
            Command::Increment => {
                memory[pointer] = memory[pointer].wrapping_add(1);
            }

            Command::Decrement => {
                memory[pointer] = memory[pointer].wrapping_sub(1);
            }

            Command::MoveLeft => {
                pointer = pointer.saturating_sub(1);
            }

            Command::MoveRight => {
                pointer = pointer.saturating_add(1);
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
