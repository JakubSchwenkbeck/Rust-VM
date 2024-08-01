
    use std::{io::{self, Write}, process::Command};

    use crate::{instructions::instructions_regs::reg_printall, interpreter::{assembler::parse_line, decoder::decode}, operating_system::memory_manager::{get_chunk_size, get_file_size, get_unique_filenames, load_program, mem_alloc, mem_release, print_unique_filenames, run_program}, Machine};

    pub fn cmd_line_interface(mach : &mut Machine) {
        
        Command::new("cmd")
        .args(&["/C", "cls"])
        .status()
        .unwrap();
        // start booting...

        // End booting....
        println!("-16 Bit Virtual Machine in Rust-");
        println!(" ");
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
                    print_help();


                  
                } 
                _ if input.starts_with("vm load ") => {
                    let filename = input.strip_prefix("vm load ").unwrap();
                    mem_alloc(filename,get_file_size(filename));
                    
                    let p =load_program(mach, filename);
                    if p{
                        println!("Succesfully loaded {filename}");
                    }else {
                        println!("Loading was not successfull : {p:?}");
                    }
                    

                }
                _ if input.starts_with("vm run ") => {
                    let remaining_input = input.strip_prefix("vm run ").unwrap();
                    let  filename: &str;
                    let  dispflag :bool;
                    if remaining_input.starts_with("-disp ") {

                         filename = remaining_input.strip_prefix("-disp ").unwrap();

                         dispflag = true;

                    } else {
                         filename = remaining_input;
                         dispflag = false;
                    }
                    let p = run_program(mach, filename,dispflag);

                    if p.is_ok(){
                        println!("Succesfully executed {filename}");
                    }else {
                        println!("Execution was not successfull : {p:?}");
                    }
                    

                }
                _ if input.starts_with("vm parse ") => {
                    let filename = input.strip_prefix("vm parse ").unwrap();
                    let p =load_program(mach, filename);
                    if p{
                        println!("Succesfully parsed {filename}");
                    }else {
                        println!("Parsing was not successfull : {p:?}");
                    }
                    

                }
                _ if input.starts_with("vm exec ") => {
                    let filename = input.strip_prefix("vm exec ").unwrap();
                    let p =  run_program(mach,filename,true);
                    if p.is_ok(){
                        println!("Succesfully executed {filename}");
                    }else {
                        println!("Execution was not successfull : {p:?}");
                    }
                }
               
                _ if input.starts_with("cls") ||input.starts_with("clear")||input.starts_with("clc")  => {
                    Command::new("cmd")
                    .args(&["/C", "cls"])
                    .status()
                    .unwrap();
                 println!("-16 Bit Virtual Machine in Rust-");
                println!(" ");
        
                }
                _ if input.starts_with("vm instr ") => {
                    let filename = input.strip_prefix("vm instr ").unwrap();
                   
                   let word = parse_line(filename,mach,true);
                      decode(word, mach, 0,true);
                    reg_printall(mach);
        
                }
                _ if input.starts_with("printmem ") => {
                    let l = input.strip_prefix("printmem ").unwrap();
                    let length = l.parse::<u16>().unwrap();
                   for i in 1..length{
                        let val = mach.memory.read(i).unwrap();
                        print!("Memory {i} = {val}  ");
                   }
                }
                _ if input.starts_with("vm malloc ") => {
                    let filename = input.strip_prefix("vm malloc ").unwrap();
                    mem_alloc(filename,get_file_size(filename));
                }
                _ if input.starts_with("vm release ") => {
                    let filename = input.strip_prefix("vm release ").unwrap();
                    mem_release(filename);
                }
                _ if input.starts_with("show config")=>{

                        let chunkconfig = get_chunk_size();
                        let memsize = mach.get_mem_size();
                        let regnum = mach.registers.len();

                  println!("Configs:");
                  println!("-System : 16 Bit");
                  println!("-Memory size : {memsize}b");
                  println!("--Chunk size : {chunkconfig}b");
                  println!("-Number of Registers: {regnum}")

                }
                _ if input.starts_with("ls")=>{ 

                    let filenames = get_unique_filenames();
                    println!("Listing of all programs and data in memory:");
                    println!("(1) - Operating System");
                    print_unique_filenames(filenames);


                  }

                _ => {
                    println!("Unknown command: '{}'", input);
                    println!("Type 'help' to see available commands.");
                }
            }
        }
    
        println!("Goodbye!");
    }



    
    pub fn print_help(){
        println!("");
        println!("Commands:\n");

        println!("General:");
        println!("  help                  Display this help message");
        println!("  exit                  Exit the CLI");
        println!("  clear, cls , clc      Clear the terminal screen");
        println!("  ls                    List all loaded programs/data\n");
        
        println!("Virtual Machine:");
        println!("  vm load <file>        Load file from host src folder (malloc and parse)");
        println!("    - malloc <file>     Allocate memory space for file");
        println!("    - parse <file>      Parse the file");
        println!("  vm release <file>     Release the Memory used by file");
        println!("  vm run <file>         Run program from memory");
        println!("  vm run -disp <file>   Run program from memory and display states");
        println!("    - exec <file>       Execute the file\n");
        
        println!("Assembler:");
        println!("  instr <instruction>   Execute a single assembler instruction\n");
        
        println!("Memory:");
        println!("  printmem <length>     Print first <length> memory spaces\n");
        
        println!("Configuration:");
        println!("  show config           Display current configurations");
        println!("  set config            Change system configurations");
        

    }