
    use std::{io::{self, Write}, process::Command};

    use crate::{instructions::instructions_regs::reg_printall, interpreter::{assembler::parse_line, decoder::decode, parse_exec::{parse_programm, run_programm}}, Machine};

    pub fn cmd_line_interface(mach : &mut Machine) {
        
        Command::new("cmd")
        .args(&["/C", "cls"])
        .status()
        .unwrap();
        // start booting...

        // End booting....
        println!("Welcome to my CLI!");
        println!("Type 'help' to see available commands.");

        loop {
            print!("> ");
            io::stdout().flush().unwrap(); // Make sure the prompt is displayed
    
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            
            let input = input.trim(); // Remove trailing newline
            if input == "exit" {
                break;
            }
            
            match input {
                "help" => {
                    println!("Available commands:");
                    println!("help - Show this help message");
                    println!("exit - Exit the CLI");
                    println!("clear / cls - clears Terminal");
                    println!("parse <Filename> - trying to parse the file");
                    println!("exec <Filename> - trying to execute the file");
                    println!("instr <Assembler instruction> - executes the single Assembler line");
                } 
                _ if input.starts_with("parse ") => {
                    let filename = input.strip_prefix("parse ").unwrap();
                    let p = parse_programm(mach, filename);
                    if p.is_ok(){
                        println!("Succesfully parsed {filename}");
                    }else {
                        println!("Parsing was not successfull : {p:?}");
                    }
                    

                }
                _ if input.starts_with("exec ") => {
                    let filename = input.strip_prefix("exec ").unwrap();
                    let p =  run_programm(mach,filename);
                    if p.is_ok(){
                        println!("Succesfully executed {filename}");
                    }else {
                        println!("Execution was not successfull : {p:?}");
                    }
                }
                _ if input.starts_with("clear") => {
                    Command::new("cmd")
                    .args(&["/C", "cls"])
                    .status()
                    .unwrap();
        
                }
                _ if input.starts_with("cls") => {
                    Command::new("cmd")
                    .args(&["/C", "cls"])
                    .status()
                    .unwrap();
        
                }
                _ if input.starts_with("instr ") => {
                    let filename = input.strip_prefix("instr ").unwrap();
                   
                   let word = parse_line(filename,mach);
                      decode(word, mach);
                    reg_printall(mach);
        
                }

                _ => {
                    println!("Unknown command: '{}'", input);
                    println!("Type 'help' to see available commands.");
                }
            }
        }
    
        println!("Goodbye!");
    }
    