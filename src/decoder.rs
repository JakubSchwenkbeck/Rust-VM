
use crate::{instructions_regs::*, u4::U4, Machine};
//* Interpreter for Assembler Level Register ISA */
// DECODE ON BINARY LEVEL

pub fn decode(word: u16,mach : &mut Machine)  {
    let opcode = (word >> 12) as u8 ;  // Extract the opcode (first 4 bits)
    
    match opcode {

        // I - FORMAT 

        0b0000 => { // LWI
            
            let rd = ((word >> 8) & 0xF) as u8;       // Extract the destination register (next 4 bits)
            let val = (word & 0xFF) as u8;

          reg_immediate_load_word(mach,U4::new(rd),val);
        },
        0b0001 => { // SWI
            let val = ((word >> 8) & 0xF) as u8;       // Extract the destination register (next 4 bits)
            let dest = (word & 0xFF) as u8;
            reg_immediate_store_word(mach, dest, U4::new(val));
        },
        0b0010 => { //ADD I
            let rd = U4::new(((word >> 8) & 0xF) as u8);
            let rs = U4::new(((word >> 4) & 0xF) as u8);
            let im = U4::new((word & 0xF) as u8);
          
            reg_immediate_add(mach, rd, rs, im);
           
        },
        0b0011 => { // Sub I
            let rd = U4::new(((word >> 8) & 0xF) as u8);
            let rs = U4::new(((word >> 4) & 0xF) as u8);
            let im = U4::new((word & 0xF) as u8);
          
            reg_immediate_sub(mach, rd, rs, im);
           
        },
        0b0100 => { //AND I
            let rd = U4::new(((word >> 8) & 0xF) as u8);
            let rs = U4::new(((word >> 4) & 0xF) as u8);
            let im = U4::new((word & 0xF) as u8);
          
            reg_immediate_and(mach, rd, rs, im);
           
        },
        0b0101 => { // OR I
            let rd = U4::new(((word >> 8) & 0xF) as u8);
            let rs = U4::new(((word >> 4) & 0xF) as u8);
            let im = U4::new((word & 0xF) as u8);
          
            reg_immediate_or(mach, rd, rs, im);
           
        },
        
        // R- Format:

        0b0110 => {
            // LOAD Word
            let rd: U4 = U4::new(((word >> 8) & 0xF) as u8);
            let source: u16 = word>> 4 as u16;
            reg_load_word(mach, rd, source);

        },
        
        0b0111 => { 
            // Store Word
            let rd: U4 = U4::new(((word >> 8) & 0xF) as u8);
            let source: u16 = word>> 4 as u16;
            reg_store_word(mach,  rd,source);

          
        },
        0b1000 => { // ADD
            let rd = U4::new(((word >> 8) & 0xF) as u8);
            let rs = U4::new(((word >> 4) & 0xF) as u8);
            let rt = U4::new((word & 0xF) as u8);
            println!("Im Decoding ADD {rd},{rs},{rt}");
            reg_add(mach, rd, rs, rt);
           
        },
        0b1001 => { // SUB
            let rd = U4::new(((word >> 8) & 0xF) as u8);
            let rs = U4::new(((word >> 4) & 0xF) as u8);
            let rt = U4::new((word & 0xF) as u8);
            println!("Im Decoding Sub {rd},{rs},{rt}");
            reg_sub(mach, rd, rs, rt);
           
        },
        0b1010 => {
            let rd = U4::new(((word >> 8) & 0xF) as u8);
            let rs = U4::new(((word >> 4) & 0xF) as u8);
            let rt = U4::new((word & 0xF) as u8);
          
            reg_and(mach, rd, rs, rt);
           
        },
        0b1011 => {
            let rd = U4::new(((word >> 8) & 0xF) as u8);
            let rs = U4::new(((word >> 4) & 0xF) as u8);
            let rt = U4::new((word & 0xF) as u8);
          
            reg_or(mach, rd, rs, rt);
           
        },
        0b1100 => {
            let rd = U4::new(((word >> 8) & 0xF) as u8);
            let rs = U4::new(((word >> 4) & 0xF) as u8);
            let rt = U4::new((word & 0xF) as u8);
          
            reg_branch_not_equal(mach, rt, rs, rd);
           
        },
        0b1101 => {
            let rd = U4::new(((word >> 8) & 0xF) as u8);
            let rs = U4::new(((word >> 4) & 0xF) as u8);
            let rt = U4::new((word & 0xF) as u8);
          
            reg_shift_left(mach, rd, rs, rt);
           
        },
        0b1110 => {
            let rd = U4::new(((word >> 8) & 0xF) as u8);
            let rs = U4::new(((word >> 4) & 0xF) as u8);
            let rt = U4::new((word & 0xF) as u8);
          
            reg_shift_right(mach, rd, rs, rt);
           
        },

        // JUMP :
        0b1111 => {

            let rd =  ((word & 0x0FF0) >>4 )as u8;
         
           let offset =  U4::new(((word & 0x0FF0) >>12 ) as u8);

            reg_jump(mach, rd, offset);
           
        },


        
        _ => {let _ = mach.step();}
    }
}