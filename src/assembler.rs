use crate::{decoder::decode, Machine};
use std::fs::File;
use std::io::{self, BufRead, BufReader};


pub fn concatenate_4bit_values(a: u8, b: u8, c: u8, d: u8) -> u16 {
    // Ensure each value is only 4 bits
    let a = a & 0x0F;
    let b = b & 0x0F;
    let c = c & 0x0F;
    let d = d & 0x0F;

    // Concatenate the values
    let result = ((a as u16) << 12) | ((b as u16) << 8) | ((c as u16) << 4) | (d as u16);
    result
}
pub fn read_lines_from_file(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut lines = Vec::new();
    for line in reader.lines() {
        let line = line?; // Handle potential I/O errors
        lines.push(line);
    }
    Ok(lines)
}



pub fn parse_line(line: &str,mach : &mut Machine){
    let parts: Vec<&str> = line.split_whitespace().collect();
    match parts.as_slice() {
        ["add", dst, src1, src2] => {
            let dst = dst.parse::<u8>().unwrap();
            let src1 =  src1.parse::<u8>().unwrap();
            let src2 =  src2.parse::<u8>().unwrap();

           let word = concatenate_4bit_values(0b1000, dst, src1,src2);
           let bin = format!("{:016b}", word);
           println!("Im adding : {word}, {bin}");
            decode(word, mach)
         
        },
        ["sub", dst, src1, src2] => {
            let dst = dst.parse::<u8>().unwrap();
            let src1 =  src1.parse::<u8>().unwrap();
            let src2 =  src2.parse::<u8>().unwrap();

           let word = concatenate_4bit_values(0b1001, dst, src1,src2);
            decode(word, mach)
         
        },
        ["and", dst, src1, src2] => {
            let dst = dst.parse::<u8>().unwrap();
            let src1 =  src1.parse::<u8>().unwrap();
            let src2 =  src2.parse::<u8>().unwrap();

           let word = concatenate_4bit_values(0b1010, dst, src1,src2);
            decode(word, mach)
         
        },
        ["or", dst, src1, src2] => {
            let dst = dst.parse::<u8>().unwrap();
            let src1 =  src1.parse::<u8>().unwrap();
            let src2 =  src2.parse::<u8>().unwrap();

           let word = concatenate_4bit_values(0b1011, dst, src1,src2);
            decode(word, mach)
         
        },
        ["sw", dst, src1] => {
            let dst = dst.parse::<u8>().unwrap();
            let src =  src1.parse::<u16>().unwrap();
            let src1 = (src >> 8) as u8; 
            let src2 = (src & 0xFF) as u8;
          
           let word = concatenate_4bit_values(0b0110, dst, src1,src2);
            decode(word, mach)
         
         
        },
        ["lw", dst,  src1] => {
            let dst = dst.parse::<u8>().unwrap();
            let src =  src1.parse::<u16>().unwrap();
            let src1 = (src >> 8) as u8; 
            let src2 = (src & 0xFF) as u8;
          
           let word = concatenate_4bit_values(0b0110, dst, src1,src2);
            decode(word, mach)
         
        },
       
       
        _ => println!("Error"),


    }
}

