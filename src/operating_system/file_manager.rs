use crate:: Machine;


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



pub fn create_file(_filename: &str,ending: &str){

if ending == ".txt"{
    // allocate memory:


    // create head:


    // encode each char into u8


    // create tail



}

}

pub fn fill_file(ch: char)
{
   

// Ensure the character is an ASCII character
if ch.is_ascii() {
    // Convert the character to a u8
    let encoded: u8 = ch as u8;
    println!("Encoded value of '{}' is {}", ch, encoded);
} else {
    println!("Character '{}' is not in the ASCII range", ch);
}
    

}
