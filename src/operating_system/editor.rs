use std::io::{self, Write};

use crate::Machine;
use crate::operating_system::file_manager::{display_file, write_file, fill_file};

use super::memory_manager::get_all_addresses;

pub fn open_editor(mach: &mut Machine, filename: &str) {
    let mut buffer = load_existing_content(mach, filename);
    let mut modified = false;

    loop {
        display_buffer(&buffer);

        print!(">>> "); // Display a prompt
        io::stdout().flush().unwrap(); // Ensure the prompt is shown

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input_trim = input.trim(); // Trim the input

        match input_trim {
            "exit" => {
                if modified {
                    println!("You have unsaved changes. Use 'save', 'save and exit', or 'exit without saving'.");
                } else {
                    break;
                }
            }
            "save" => {
                println!("Saving the file...");
                save_buffer(mach, &buffer, filename);
                modified = false; // Reset the modification flag
                println!("File saved.");
            }
            "save and exit" => {
                println!("Saving the file and exiting...");
                save_buffer(mach, &buffer, filename);
                modified = false; // Reset the modification flag
                break;
            }
            "exit without saving" => {
                println!("Exiting without saving changes...");
                break;
            }
            "clear" => {
                clear_screen();
            }
            _ => {
                // Edit the buffer content
                edit_buffer(&mut buffer, input_trim.to_string());
                modified = true; // Mark that the file has been modified
            }
        }
    }
}

fn load_existing_content(mach: &mut Machine, filename: &str) -> Vec<String> {
    let mut buffer = Vec::new();
    let addresses = get_all_addresses(filename);

    let mut line = String::new();
    for &adr in &addresses {
        let c = mach.memory.read(adr).unwrap() as char;
        if c == '\0' {
            break; // Stop reading at the first null character
        }
        if c == '\n' {
            buffer.push(line.clone());
            line.clear();
        } else {
            line.push(c);
        }
    }

    if !line.is_empty() {
        buffer.push(line);
    }
    buffer
}

fn display_buffer(buffer: &Vec<String>) {
    clear_screen();
    println!("--- Editing ---\n");
    for (i, line) in buffer.iter().enumerate() {
        println!("{}: {}", i + 1, line);
    }
}

fn edit_buffer(buffer: &mut Vec<String>, input: String) {
    let mut parts = input.splitn(2, ' '); // Split the input into command and content
    let command = parts.next().unwrap_or_default();
    let content = parts.next().unwrap_or_default().to_string();

    if command.starts_with(':') {
        if let Ok(line_number) = command[1..].parse::<usize>() {
            if line_number > 0 && line_number <= buffer.len() {
                buffer[line_number - 1] = content;
            } else if line_number == buffer.len() + 1 {
                buffer.push(content);
            } else {
                println!("Line number out of bounds.");
            }
        } else {
            println!("Invalid line number.");
        }
    } else {
        buffer.push(input); // Append the input as a new line
    }
}

fn save_buffer(mach: &mut Machine, buffer: &Vec<String>, filename: &str) {
    let addresses = get_all_addresses(filename);
    let mut index = 0;

    for line in buffer {
        for ch in line.chars() {
            if index < addresses.len() {
                fill_file(mach, ch, addresses[index]);
                index += 1;
            } else {
                println!("Error: Not enough memory to save the file.");
                return;
            }
        }
        if index < addresses.len() {
            fill_file(mach, '\n', addresses[index]);
            index += 1;
        } else {
            println!("Error: Not enough memory to save the file.");
            return;
        }
    }

    // Fill the rest of the space with null characters (if any space is left)
    while index < addresses.len() {
        fill_file(mach, '\0', addresses[index]);
        index += 1;
    }
}

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    io::stdout().flush().unwrap();
    println!("Screen cleared.");
}
