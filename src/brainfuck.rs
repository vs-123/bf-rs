use std::io::Read;

#[derive(Debug, Clone)]
pub enum Command {
    Increment,
    Decrement,
    MoveLeft,
    MoveRight,
    LoopOpen(usize), // usize -> End of loop
    LoopClose(usize), // usize -> Start of loop
    PrintCell,
    InputCell,
}

pub fn parse(source: &str) -> Vec<Command> {
    let mut output = Vec::<Command>::new();
    let mut loop_stack = Vec::new();

    for (character_index, character) in source.chars().enumerate() {
        match character {
            '+' => output.push(Command::Increment),
            '-' => output.push(Command::Decrement),
            '<' => output.push(Command::MoveLeft),
            '>' => output.push(Command::MoveRight),

            '[' => {
                let loop_start = output.len();
                output.push(Command::LoopOpen(character_index));
                loop_stack.push(loop_start);
            }

            ']' => {
                if let Some(loop_start) = loop_stack.pop() {
                    output[loop_start] = Command::LoopOpen(output.len());
                    output.push(Command::LoopClose(loop_start));
                } else {
                    println!("\n[Error at command index {}]", output.len());
                    println!("Unmatched ']'");
                    std::process::exit(1);
                }
            }

            '.' => output.push(Command::PrintCell),
            ',' => output.push(Command::InputCell),

            // Ignore other characters
            _ => {}
        }
    }

    return output
}

pub fn interpret(commands: &[Command]) {
    let mut memory = [0u8; 30_000];
    let mut pointer = 0usize;
    let mut command_index = 0;

    while command_index < commands.len() {
        let command = commands[command_index].clone();

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
                print!("{}", memory[pointer] as char);
            }

            Command::InputCell => {
                let mut buf = [0; 1];
                std::io::stdin().read(&mut buf).unwrap();
                memory[pointer] = buf[0];
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
