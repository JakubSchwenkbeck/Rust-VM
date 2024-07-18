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
        ["jump", addr, offset] => {
            let dst1 = addr.parse::<u8>().unwrap()  ;
            let dst2 = addr.parse::<u8>().unwrap();
            
            let off =  offset.parse::<u8>().unwrap();

           let word = concatenate_4bit_values(0b1111, dst1,dst2, off);
            decode(word, mach)
         
        },

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
          

            let bin = format!("{:016b}", word);
            println!("Im subbing : {word}, {bin}");
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
        ["bne", src1, src2,dest] => {

            let dst = dest.parse::<u8>().unwrap();
            let comp1 =  src1.parse::<u8>().unwrap();
            let cmop2 =  src2.parse::<u8>().unwrap();

           let word = concatenate_4bit_values(0b1100, comp1, cmop2,dst);
           let bin = format!("{:016b}", word);
           println!("Im checking branch  : {word}, {bin}");
            decode(word, mach)
         
        },
        ["sl", dst, src1, src2] => {
            let dst = dst.parse::<u8>().unwrap();
            let src1 =  src1.parse::<u8>().unwrap();
            let src2 =  src2.parse::<u8>().unwrap();

           let word = concatenate_4bit_values(0b1101, dst, src1,src2);
           let bin = format!("{:016b}", word);
           println!("Im shifting left : {word}, {bin}");
            decode(word, mach)
         
        },
        ["sr", dst, src1, src2] => {
            let dst = dst.parse::<u8>().unwrap();
            let src1 =  src1.parse::<u8>().unwrap();
            let src2 =  src2.parse::<u8>().unwrap();

           let word = concatenate_4bit_values(0b1110, dst, src1,src2);
           let bin = format!("{:016b}", word);
           println!("Im shifting right : {word}, {bin}");
            decode(word, mach)
         
        },

            // I Format

        ["lwi", dst, imm] => {
                let dst = dst.parse::<u8>().unwrap();
               
                let imm =  imm.parse::<u8>().unwrap();
    
               let word = concatenate_4bit_values(0b0010, dst,imm,0b000);
               let bin = format!("{:016b}", word);
               println!("Im laoding immediate : {word}, {bin}");
                decode(word, mach)
             
            }, ["swi", dst, imm] => {
                let dst = dst.parse::<u8>().unwrap();
                
                let imm =  imm.parse::<u8>().unwrap();
    
               let word = concatenate_4bit_values(0b0010, dst,imm, 0b000);
               let bin = format!("{:016b}", word);
               println!("Im storing immediate : {word}, {bin}");
                decode(word, mach)
             
            },


        ["addi", dst, src1, imm] => {
            let dst = dst.parse::<u8>().unwrap();
            let src1 =  src1.parse::<u8>().unwrap();
            let imm =  imm.parse::<u8>().unwrap();

           let word = concatenate_4bit_values(0b0010, dst, src1,imm);
           let bin = format!("{:016b}", word);
           println!("Im adding immediate : {word}, {bin}");
            decode(word, mach)
         
        },
        ["subi", dst, src1, imm] => {
            let dst = dst.parse::<u8>().unwrap();
            let src1 =  src1.parse::<u8>().unwrap();
            let imm: u8 =  imm.parse::<u8>().unwrap();

           let word = concatenate_4bit_values(0b0011, dst, src1,imm);
           let bin = format!("{:016b}", word);
           println!("Im subing immediate : {word}, {bin}");
            decode(word, mach)
         
        },
        ["andi", dst, src1, imm] => {
            let dst = dst.parse::<u8>().unwrap();
            let src1 =  src1.parse::<u8>().unwrap();
            let imm: u8 =  imm.parse::<u8>().unwrap();

           let word = concatenate_4bit_values(0b0100, dst, src1,imm);
           let bin = format!("{:016b}", word);
           println!("Im and-ing immediate : {word}, {bin}");
            decode(word, mach)
         
        },
        ["ori", dst, src1, imm] => {
            let dst = dst.parse::<u8>().unwrap();
            let src1 =  src1.parse::<u8>().unwrap();
            let imm: u8 =  imm.parse::<u8>().unwrap();

           let word = concatenate_4bit_values(0b0101, dst, src1,imm);
           let bin = format!("{:016b}", word);
           println!("Im or-ing immediate : {word}, {bin}");
            decode(word, mach)
         
        },

       
       
        _ => println!("Error"),


    }
}

