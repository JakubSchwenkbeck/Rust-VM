use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;

const CHUNK_SIZE: u16 = 16; // Define the size of each chunk


// static data structures
lazy_static! {
    static ref HASHMAP: Mutex<HashMap<String, Vec<Chunk>>> = Mutex::new(HashMap::new());
    static ref FREE_LIST: Mutex<Vec<Chunk>> = Mutex::new(vec![]);
}

// keep track of latest adress (obsolet bc of pc)
static mut LATEST_ADDRESS: u16 = 0;


// def chunk
#[derive(Copy, Clone, Debug)]
pub struct Chunk {
    pub start: u16,
    pub end: u16,
}

impl Chunk {
    pub fn new(start: u16) -> Self {
        Self {
            start,
            end: start + CHUNK_SIZE - 1,
        }
    }
}

// config getters and setters:

pub fn get_chunk_size() -> u16{CHUNK_SIZE} // not changable





// ls :
pub fn get_unique_filenames() -> Vec<String> {
    let hashmap = HASHMAP.lock().unwrap();
    let mut filenames: Vec<String> = hashmap.keys().cloned().collect();
    filenames.sort(); // Optional: sort the filenames
    filenames.dedup(); // Remove duplicates, if any
    filenames
}

// Function to print all unique filenames
pub fn print_unique_filenames(filenames:Vec<String>) {
    let mut counter = 1;
    for filename in filenames {
        counter = counter +1;
        println!("({counter})- {}", filename);
    }
}


pub fn get_latest_addr() -> u16 {
    unsafe { LATEST_ADDRESS }
}

pub fn mem_alloc(filename: &str, size: u16) -> bool {
    let mut map = HASHMAP.lock().unwrap();
    let mut free_list = FREE_LIST.lock().unwrap();

    if map.contains_key(filename) {
        println!("This file already allocated memory space");
        return false;
    }

    let mut chunks: Vec<Chunk> = vec![];
    let mut remaining_size = size;

    while remaining_size > 0 {
        if let Some(free_chunk) = free_list.pop() {
            chunks.push(free_chunk);
            remaining_size -= CHUNK_SIZE.min(remaining_size);
        } else {
            let new_chunk = Chunk::new(unsafe { LATEST_ADDRESS });
            unsafe { LATEST_ADDRESS += CHUNK_SIZE };
            chunks.push(new_chunk);
            remaining_size -= CHUNK_SIZE.min(remaining_size);
        }
    }

    map.insert(filename.to_owned(), chunks);
    true
}

pub fn mem_release(filename: &str) {
    let mut map = HASHMAP.lock().unwrap();
    let mut free_list = FREE_LIST.lock().unwrap();

    if let Some(chunks) = map.remove(filename) {
        for chunk in chunks {
            free_list.push(chunk);
        }
        free_list.sort_by_key(|c| c.start);
        let mut merged_list = vec![];
        let mut current = free_list[0];
        for &chunk in free_list.iter().skip(1) {
            if current.end + 1 == chunk.start {
                current.end = chunk.end;
            } else {
                merged_list.push(current);
                current = chunk;
            }
        }
        merged_list.push(current);
        *free_list = merged_list;
        println!("Memory released and merged: {:?}", free_list);
    } else {
        println!("No memory allocated for this file");
    }
}

// Mock function to simulate file size
pub fn get_file_size(filename: &str) -> u16 {
    match count_non_empty_lines(filename) {
        Ok(count) => count,
        Err(e) => {
            println!("Error: {}", e);
            0
        }
    }
}

use std::fs::File;
use std::io::{self, BufRead};

use crate::instructions::instructions_regs::{reg_printall, reg_single_print};
use crate::interpreter::assembler::{parse_line, read_lines_from_file};
use crate::interpreter::decoder::decode;
use crate::u4::U4;
use crate::Machine;

fn count_non_empty_lines(filename: &str) -> io::Result<u16> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let count = reader
        .lines()
        .filter_map(Result::ok)
        .filter(|line| !line.trim().is_empty())
        .count();

    Ok(count as u16)
}

pub fn load_program(mach: &mut Machine, filename: &str) -> bool {
    let map = HASHMAP.lock().unwrap();
    
    // Check if the program is allocated in memory
    if let Some(chunks) = map.get(filename) {
        // Read all the program data at once
        let program_data = read_program_data(mach, filename);
        
        let mut addr = chunks[0].start;
        let mut data_index = 0; // Index for accessing program_data

        // Iterate through the allocated chunks
        for chunk in chunks.iter() {
            let chunk_size = (chunk.end - chunk.start + 1) as usize; // Calculate the size of the current chunk
            let mut bytes_written = 0; // Keep track of how many bytes we've written to the current chunk

            // Write to the current chunk until it's full or there is no more data
            while bytes_written < chunk_size && data_index < program_data.len() {
                // Write each byte to the memory
                mach.memory.write2(addr, program_data[data_index]);
                addr += 2; // Move to the next address (2 bytes per instruction)
                bytes_written += 2; // Increment the count of bytes written
                data_index += 1; // Move to the next program data
            }

            // If we've exhausted the program data, we can stop writing
            if data_index >= program_data.len() {
                break;
            }
        }
        true
    } else {
        false
    }
}


fn read_program_data(mach: &mut Machine, filename: &str) -> Vec<u16> {
    let lines =read_lines_from_file(&filename).unwrap();

    let mut res: Vec<u16>= Vec::new(); 
    for line in lines{
        res.push(parse_line(&line,   mach,false))
    
    }
    res
}
pub fn run_program(virtualm: &mut Machine, filename: &str,dispflag: bool) -> Result<(), &'static str> {
    // Load the program into the virtual machine memory
  
    // Reset the virtual machine's registers except the program counter (PC)
    virtualm.reset_registers_except_pc();


 // Retrieve the allocated chunks for the program
 let map = HASHMAP.lock().unwrap();
 let chunks = map.get(filename).ok_or("Program not found in memory")?;
 let start = chunks[0].start;
 let end = chunks.last().unwrap().end;

if dispflag{ println!("Start {start}, End {end}");}

 // Set the initial program counter (PC) to the start address
 virtualm.registers[13] = start;

 // Main execution loop
 while let Some(l) = virtualm.memory.read2(virtualm.registers[13]) {
     // Decode and execute the instruction
     decode(l, virtualm, start,dispflag);
    if dispflag{
     // Print current state for debugging
     let current_pc = virtualm.registers[13];
     println!("current: {current_pc}");
     println!("{l}");
     reg_printall(virtualm);
    }
     // Advance the program counter (PC) by 2 (since each instruction is 2 bytes)
     virtualm.registers[13] += 2;

     // Check if we have reached the end of the allocated chunks
     if virtualm.registers[13] > end {
         break;
     }
 }

 // Print the final state of the specific register
 reg_single_print(virtualm, U4::new(15));

 Ok(())
}