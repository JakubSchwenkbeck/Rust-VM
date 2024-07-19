
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use crate::Machine;


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



pub fn parse_line(line: &str,mach :  &mut Machine) -> u16{
    let parts: Vec<&str> = line.split_whitespace().collect();
    match parts.as_slice() {
        ["jump", addr, offset] => {
            let dst = addr.parse::<u8>().unwrap()  ;

            let dst1 = dst >> 4;
            let dst2 = dst & 0x0F;
           
            let off =  offset.parse::<u8>().unwrap();

           let word = concatenate_4bit_values(0b1111, dst1,dst2, off);
            
            //decode(word, mach);
            return word;
         
        },

        ["add", dst, src1, src2] => {
            let dst = dst.parse::<u8>().unwrap();
            let src1 =  src1.parse::<u8>().unwrap();
            let src2 =  src2.parse::<u8>().unwrap();

           let word = concatenate_4bit_values(0b1000, dst, src1,src2);
           let bin = format!("{:016b}", word);
           println!("Im adding : {word}, {bin}");
            //decode(word, mach)
            return word;
        },
        ["sub", dst, src1, src2] => {
            let dst = dst.parse::<u8>().unwrap();
            let src1 =  src1.parse::<u8>().unwrap();
            let src2 =  src2.parse::<u8>().unwrap();

           let word = concatenate_4bit_values(0b1001, dst, src1,src2);
          

            let bin = format!("{:016b}", word);
            println!("Im subbing : {word}, {bin}");
            //decode(word, mach)
            return word;
        },
        ["and", dst, src1, src2] => {
            let dst = dst.parse::<u8>().unwrap();
            let src1 =  src1.parse::<u8>().unwrap();
            let src2 =  src2.parse::<u8>().unwrap();

           let word = concatenate_4bit_values(0b1010, dst, src1,src2);
            //decode(word, mach)
            return word;
        },
        ["or", dst, src1, src2] => {
            let dst = dst.parse::<u8>().unwrap();
            let src1 =  src1.parse::<u8>().unwrap();
            let src2 =  src2.parse::<u8>().unwrap();

           let word = concatenate_4bit_values(0b1011, dst, src1,src2);
            //decode(word, mach)
            return word;
        },
        ["sw", src, dst] => {
            let src = src.parse::<u8>().unwrap();
            let dst =  dst.parse::<u8>().unwrap();
            let dst1 = (dst >> 4) as u8; 
            let dst2 = (dst & 0x0F) as u8;
           let word = concatenate_4bit_values(0b0111, src, dst1,dst2);
           let bin = format!("{:016b}", word);
           println!("Im saving : {word}, {bin}");
            //decode(word, mach)
            return word;
         
        },
        ["lw", dst,  src1] => {
            let dst = dst.parse::<u8>().unwrap();
            let src =  src1.parse::<u8>().unwrap();
            let src1 = (src >> 4) as u8; 
            let src2 = (src & 0x0F) as u8;
          
           let word = concatenate_4bit_values(0b0110, dst, src1,src2);
            //decode(word, mach)
            return word;
        },
        ["bne", src1, src2,dest] => {

            let dst = dest.parse::<u8>().unwrap();
            let comp1 =  src1.parse::<u8>().unwrap();
            let comp2 =  src2.parse::<u8>().unwrap();

           let word = concatenate_4bit_values(0b1100, comp1, comp2,dst);
           let bin = format!("{:016b}", word);
           println!("Im checking branch  : {word}, {bin}");
            //decode(word, mach)
            return word;
        },
        ["sl", dst, src1, src2] => {
            let dst = dst.parse::<u8>().unwrap();
            let src1 =  src1.parse::<u8>().unwrap();
            let src2 =  src2.parse::<u8>().unwrap();

           let word = concatenate_4bit_values(0b1101, dst, src1,src2);
           let bin = format!("{:016b}", word);
           println!("Im shifting left : {word}, {bin}");
            //decode(word, mach)
            return word;
        },
        ["sr", dst, src1, src2] => {
            let dst = dst.parse::<u8>().unwrap();
            let src1 =  src1.parse::<u8>().unwrap();
            let src2 =  src2.parse::<u8>().unwrap();

           let word = concatenate_4bit_values(0b1110, dst, src1,src2);
           let bin = format!("{:016b}", word);
           println!("Im shifting right : {word}, {bin}");
            //decode(word, mach)
            return word;
        },

            // I Format

        ["lwi", dst, imm] => {
                let dst = dst.parse::<u8>().unwrap();
               
                let imm =  imm.parse::<u8>().unwrap();

                let imm1 = imm >> 4;
                let imm2 = imm & 0x0F;
            
    
               let word = concatenate_4bit_values(0b0000, dst,imm1,imm2);
               let bin = format!("{:016b}", word);
               println!("Im loading immediate : {word}, {bin}");
                //decode(word, mach)
                return word;
            }, ["swi", dst, imm] => {  
                let dst = dst.parse::<u8>().unwrap();
                let dst1 = dst >> 4;
                let dst2 = dst & 0x0F;
                let imm =  imm.parse::<u8>().unwrap();
                
            
               let word = concatenate_4bit_values(0b0001, dst1,dst2, imm);
               let bin = format!("{:016b}", word);
               println!("Im storing immediate : {word}, {bin}");
                //decode(word, mach)
                return word;
            },


        ["addi", dst, src1, imm] => {
            let dst = dst.parse::<u8>().unwrap();
            let src1 =  src1.parse::<u8>().unwrap();
            let imm =  imm.parse::<u8>().unwrap();

           let word = concatenate_4bit_values(0b0010, dst, src1,imm);
           let bin = format!("{:016b}", word);
           println!("Im adding immediate : {word}, {bin}");
            //decode(word, mach)
            return word;
        },
        ["subi", dst, src1, imm] => {
            let dst = dst.parse::<u8>().unwrap();
            let src1 =  src1.parse::<u8>().unwrap();
            let imm: u8 =  imm.parse::<u8>().unwrap();

           let word = concatenate_4bit_values(0b0011, dst, src1,imm);
           let bin = format!("{:016b}", word);
           println!("Im subing immediate : {word}, {bin}");
            //decode(word, mach)
            return word;
        },
        ["andi", dst, src1, imm] => {
            let dst = dst.parse::<u8>().unwrap();
            let src1 =  src1.parse::<u8>().unwrap();
            let imm: u8 =  imm.parse::<u8>().unwrap();

           let word = concatenate_4bit_values(0b0100, dst, src1,imm);
           let bin = format!("{:016b}", word);
           println!("Im and-ing immediate : {word}, {bin}");
            //decode(word, mach)
            return word;
        },
        ["ori", dst, src1, imm] => {
            let dst = dst.parse::<u8>().unwrap();
            let src1 =  src1.parse::<u8>().unwrap();
            let imm: u8 =  imm.parse::<u8>().unwrap();

           let word = concatenate_4bit_values(0b0101, dst, src1,imm);
           let bin = format!("{:016b}", word);
           println!("Im or-ing immediate : {word}, {bin}");
            //decode(word, mach)
            return word;
         
        },

       /*["print", dst]=>{
                        
            return 32727;
        } 
        */
       
       
        _ => {println!("Finished Reading");
        return 32727;}


    }
}

