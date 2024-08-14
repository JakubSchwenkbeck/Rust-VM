use std::io::{self, Write};

use crate::Machine;
use crate::operating_system::file_manager::{display_file, write_file};

pub fn open_editor(mach: &mut Machine, filename: &str) {
    // Load and display the initial content
    println!("Welcome to the Editor✨✍️   Currently Working on: {filename}\n");
    display_file(mach, filename);

    let mut start = 0;
    let mut modified = false;

    loop {
        print!(">>> "); // Display a prompt
        io::stdout().flush().unwrap(); // Ensure the prompt is shown

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input_trim = input.trim(); // Trim the input

        if input_trim == "exit" {
            if modified {
                println!("You have unsaved changes. Use 'save' or 'save and exit'.");
            } else {
                break;
            }
        } else if input_trim == "save" {
            println!("Saving the file...");
            start = write_file(mach, &input, filename, start);
            modified = false; // Reset the modification flag
            println!("File saved.");
        } else if input_trim == "save and exit" {
            println!("Saving the file and exiting...");
            start = write_file(mach, &input, filename, start);
            modified = false; // Reset the modification flag
            break;
        } else if input_trim == "clear" {
            clear_screen();
        } else {
            // Append the input to the file
            let last = write_file(mach, &input, filename, start);
            start = last;
            modified = true; // Mark that the file has been modified
        }
    }
}

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    io::stdout().flush().unwrap();
    println!("Screen cleared.");
}
