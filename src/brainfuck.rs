use std::io::{self, Read, Write};

#[derive(Debug, Clone, Copy)]
pub enum Command {
    Increment(isize),
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
                if let Some(Command::Increment(count)) = output.last_mut() {
                    *count -= 1;
                } else {
                    output.push(Command::Increment(-1));
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
                    eprintln!("\n[Error at command index {}]", output.len());
                    eprintln!("Unmatched ']'");
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
    let mut pointer: isize = 0;
    let commands_len = commands.len();

    let mut stdout = io::stdout();
    let mut stdin = io::stdin();

    let mut command_index = 0;
    while command_index < commands_len {
        let command = unsafe { commands.get_unchecked(command_index) };

        match command {
            Command::Increment(count) => {
                let mem_val = unsafe { memory.get_unchecked_mut(pointer as usize) };
                *mem_val = mem_val.wrapping_add(*count as u8);
            }
            Command::MoveLeft(count) => pointer -= *count as isize,
            Command::MoveRight(count) => pointer += *count as isize,
            Command::PrintCell => {
                let mem_val = unsafe { memory.get_unchecked(pointer as usize) };
                stdout.write(&[*mem_val]).and(stdout.flush()).ok();
            }
            Command::InputCell => {
                let mem_val = unsafe { memory.get_unchecked_mut(pointer as usize) };
                stdin.read(&mut [*mem_val]).ok();
            }
            Command::LoopOpen(loop_end) => {
                let mem_val = unsafe { memory.get_unchecked(pointer as usize) };
                if *mem_val == 0 {
                    command_index = *loop_end;
                }
            }
            Command::LoopClose(loop_start) => {
                let mem_val = unsafe { memory.get_unchecked(pointer as usize) };
                if *mem_val != 0 {
                    command_index = *loop_start;
                }
            }
        }

        command_index += 1;
    }
}
