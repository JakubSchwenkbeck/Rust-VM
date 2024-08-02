use crate:: Machine;

use super::memory_manager::{get_all_addresses, get_latest_addr, mem_alloc};


/* Magic Numbers for file types:
 -00001111 is .txt

    other files:
 -11110000 .asm
           .raw
 
  */




pub fn store_text<'a>(mach:&'a mut Machine, text: &'a str, start_address: usize) -> Result<usize, &'a str> {
    let bytes = text.as_bytes();

    if start_address + bytes.len() >mach.get_mem_size() {
        return Err("Not enough memory to store text.");
    }

    for (i, &byte) in bytes.iter().enumerate() {
        mach.memory.write((start_address + i) as u16, byte);
    }
    Ok(start_address + bytes.len())
}

pub fn read_text(mach:&mut Machine, start_address: usize, length: usize) -> String {
    let mut text = Vec::new();
    for i in 0..length {
        let byte = mach.memory.read((start_address + i) as u16).unwrap();
        text.push(byte);
    }
    String::from_utf8_lossy(&text).to_string()
}

const DEFAULT_ALLOC_SIZE: i32 = 256; // 128?

pub fn create_file(mach:&mut Machine,filename: &str,ending: &str){

if ending == ".txt"{
    // allocate memory:
    let start = mem_alloc(filename, DEFAULT_ALLOC_SIZE as u16);
    
    // create head:
    mach.memory.write(start, 0b00001111);


    // encode each char into u8


    // create tail
    let tail = get_latest_addr();
    mach.memory.write(tail, 0b0000000);


}

 println!("\n Successfully created {filename} \n");

}
pub fn write_file(mach:&mut Machine,input : &str, filename: &str,start : u16)->u16{

    let chars: Vec<char> = input.chars().collect();
    let addresses = get_all_addresses(filename);
    let mut index = start;
    for c in chars{
        
    
        match addresses.get(index as usize) {
            Some(&address) =>  {
                fill_file(mach,c,address);
                index += 1},

            None => println!("Out of Bounds!!"),
        }
    
    }
    index

}

pub fn fill_file(mach:&mut Machine,ch: char,adress: u16)
{
   

// Ensure the character is an ASCII character
if ch.is_ascii() {
    // Convert the character to a u8
    let encoded: u8 = ch as u8;
    println!("Encoded value of '{}' is {}", ch, encoded);
    mach.memory.write(adress, encoded);
    
} else {
    println!("Character '{}' is not in the ASCII range", ch);
}
    

}


pub fn display_file(mach:& mut Machine,filename: &str){
let addresses = get_all_addresses(filename);
println!("You openend {filename} : \n");
for adr in addresses{
    let c = mach.memory.read(adr).unwrap() as char;
    print!("{c}");

}
print!(" \n");


}