
pub fn create_file(filename: &str,ending: &str){

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
