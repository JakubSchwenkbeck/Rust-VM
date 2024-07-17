
use crate::memory::{LinMem,Addressable};
use crate::vm::Register;
use crate::u4::U4;

use crate::vm::Machine;

// Implementing our ISA for Register control on assembler level, mimicking a cpu in mips style

// building for 16 Registers:

// Formats : R - Type, I - Type and J - Type

// --> R - Type  4 bits OpCode, 4 bit address, 4 bit address, 4 Bit Adress XXXX-XXXX-XXXX-XXXX
        // LW 0110 - XXXX - XXXXXXXX
        // SW 0111 - XXXXXXXX - XXXX
        // Add 1000 - XXXX - XXXX - XXXX
        // Sub 1001 - XXXX - XXXX - XXXX
        // And 1010 - XXXX - XXXX - XXXX
        // Or 1011 - XXXX - XXXX - XXXX

        //bne 1100 - XXXX - XXXX - XXXX
        //shiftL 1101 - XXXX - XXXX - XXXX
        //shiftR 1110 - XXXX - XXXX - XXXX





// --> I - Type 4 bits Opcode, 4 bits adress, 4 bits adress, 4 bits immediate value XXXX-XXXX-XXXX-XXXX
        // LWI 0000 - XXXX - XXXX - 0000
        // SWI 0001 - XXXX - XXXX - 0000
        // AddI 0010 - XXXX - XXXX - XXXX
        // SubI 0011 - XXXX - XXXX - XXXX
        // AndI 0100 - XXXX - XXXX - XXXX
        // OrI 0101 - XXXX - XXXX - XXXX

// --> J - Type : 4 bits Address 
            //jump 1111 - XXXX - 0000 - 0000
        


            //* IMPLEMENTATION FOR REGISTERS *//

                // R - FORMAT
            pub fn reg_load_word( mach :&mut Machine ,dest: U4,source : u16){
                        let val  = mach.memory.read(source);
                        let mut op = 0;
                        if val.is_some(){
                                op = val.unwrap();
                        }
                        mach.registers[dest] = op as u16;

                        
            }
            pub fn reg_store_word( mach :&mut Machine ,dest:u16 ,source : U4){
                                mach.memory.write(dest, mach.registers[source] as u8);
                
            }
            pub fn reg_add_word( mach :&mut Machine ,dest: U4,source1 : U4,source2 : U4){
                
            }
            pub fn reg_sub_word( mach :&mut Machine ,dest: U4,source1 : U4,source2 : U4){
                
            }
            pub fn reg_and_word( mach :&mut Machine ,dest: U4,source1 : U4,source2 : U4){
                
            }
            pub fn reg_or_word( mach :&mut Machine ,dest: U4,source1 : U4,source2 : U4){
                
            }
            pub fn reg_branch_not_equal( mach :&mut Machine ,comp1: U4,comp2 : U4, dest: U4){

            }
            pub fn reg_shift_left( mach :&mut Machine ,dest: U4,source : U4, shift_amount : U4){

            }
            pub fn reg_shift_right( mach :&mut Machine ,dest: U4,source : U4,shift_amount : U4){
                
            }

            // I- FORMAT
            pub fn reg_immediate_load_word(mach :&mut Machine ,dest: U4,source : U4,_input :  U4){

            }
            pub fn reg_immediate_store_word(mach :&mut Machine ,dest: U4,source : U4,_input :  U4){
                
            }
            pub fn reg_immediate_add_word(mach :&mut Machine ,dest: U4,source : U4,_input :  U4){
                
            }
            pub fn reg_immediate_sub_word(mach :&mut Machine ,dest: U4,source : U4,_input :  U4){
                
            }
            pub fn reg_immediate_and_word(mach :&mut Machine ,dest: U4,source : U4,_input :  U4){
                
            }
            pub fn reg_immediate_or_word(mach :&mut Machine ,dest: U4,source : U4,_input :  U4){

            }
            
            // J - FORMAT
            pub fn reg_jump(mach :&mut Machine ,dest: U4){

            }