use std::io::{self, Read, Write};

#[derive(Debug, Clone, Copy)]
pub enum Command {
    Increment(isize),
    MovePointer(isize),
    LoopOpen(usize),  // usize -> End of loop
    LoopClose(usize), // usize -> Start of loop
    PrintCell,
    InputCell,
}

pub fn parse(source: &str) -> Vec<Command> {
    let mut output = Vec::<Command>::with_capacity(source.len());
    let mut loop_stack = Vec::new();
    let source_bytes = source.as_bytes();

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
                if let Some(Command::MovePointer(count)) = output.last_mut() {
                    *count -= 1;
                } else {
                    output.push(Command::MovePointer(-1));
                }
            }

            b'>' => {
                if let Some(Command::MovePointer(count)) = output.last_mut() {
                    *count += 1;
                } else {
                    output.push(Command::MovePointer(1));
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
            _ => {}
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
        let command = commands.get(command_index).unwrap();

        match command {
            Command::Increment(count) => {
                let mem_val = &mut memory[pointer as usize];
                *mem_val = mem_val.wrapping_add(*count as u8);
            }

            Command::MovePointer(count) => pointer += count,

            Command::PrintCell => {
                let mem_val =  &memory[pointer as usize];
                let output = if *mem_val == 10 { b'\n' } else { *mem_val };
                stdout.write(&[output]).and(stdout.flush()).ok();
            }

            Command::InputCell => {
                let mut input = [0_u8; 1];
                match stdin.read(&mut input) {
                    Ok(0) => {
                        // No input provided, set memory cell to 0
                        let mem_val = &mut memory[pointer as usize];
                        *mem_val = 0;
                    }

                    Ok(_) => {
                        let mem_val = &mut memory[pointer as usize];
                        *mem_val = if input[0] == b'\n' { 10 } else { input[0] };
                    }

                    Err(_) => {}
                }
            }

            Command::LoopOpen(loop_end) => {
                let mem_val: &mut u8 =  &mut memory[pointer as usize];
                if *mem_val == 0 {
                    command_index = *loop_end;
                } else {
                    // Check for clear loop pattern: [-] or [+]
                    let next_command = commands.get(command_index + 1).unwrap();
                    if matches!(next_command, Command::Increment(-1) | Command::Increment(1)) {
                        let next_next_command = commands.get(command_index + 2).unwrap();
                        if let Command::LoopClose(_) = next_next_command {
                            // Set memory cell to 0 after the pattern is detected
                            *mem_val = 0;
                            command_index = *loop_end;
                        }
                    }
                }
            }

            Command::LoopClose(loop_start) => {
                let mem_val =  &memory[pointer as usize];
                if *mem_val != 0 {
                    command_index = *loop_start;
                }
            }
        }

        command_index += 1;
    }
}
