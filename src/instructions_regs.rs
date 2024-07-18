

use crate::Register;
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
        // LWI 0000 - XXXX - XXXXXXXX
        // SWI 0001  - XXXXXXXX - XXXX
        // AddI 0010 - XXXX - XXXX - XXXX
        // SubI 0011 - XXXX - XXXX - XXXX
        // AndI 0100 - XXXX - XXXX - XXXX
        // OrI 0101 - XXXX - XXXX - XXXX

// --> J - Type : 4 bits Address 
            //jump 1111 - XXXX - 0000 - 0000
        

//* Util */
pub fn reg_single_print(mach: &mut Machine, dest: U4){
        let v = mach.registers[dest];
        println!("in Reg : {v}");
        let binstring =format!("{:b}", v);
        println!("As binary: {binstring}");
    
}

pub fn reg_printall(mach: &mut Machine){
    println!("");
    println!("");
    println!("-----All Registers-------");
    
    for index in 0..16 {
       let val = mach.registers[index];
       println!("Register {index} = {val}");
        
    }
    println!("");
    println!("");

}



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
            pub fn reg_store_word( mach :&mut Machine  ,source : U4,dest : u16){
                mach.memory.write(dest, mach.registers[source] as u8);

            }
            pub fn reg_add( mach :&mut Machine ,dest: U4,source1 : U4,source2 : U4){
                let val1 = mach.registers[source1];
                let val2 = mach.registers[source2];
                let sum = val1 +val2;
                mach.registers[dest] = sum;
                println!("Im Writing {sum} into reg at {dest}");

                
            }
            pub fn reg_sub( mach :&mut Machine ,dest: U4,source1 : U4,source2 : U4){
                let val1 = mach.registers[source1];
                let val2 = mach.registers[source2];
                let sum = val1 -val2;
                mach.registers[dest] = sum;
                println!("Im Writing {sum} into reg at {dest}");

            }
            pub fn reg_and( mach :&mut Machine ,dest: U4,source1 : U4,source2 : U4){
                let val1 = mach.registers[source1];
                let val2 = mach.registers[source2];
                let sum = val1 & val2;
                mach.registers[dest] = sum;
               

            }
            pub fn reg_or( mach :&mut Machine ,dest: U4,source1 : U4,source2 : U4){
                let val1 = mach.registers[source1];
                let val2 = mach.registers[source2];
                let sum = val1 | val2;
                mach.registers[dest] = sum;

            }
            pub fn reg_branch_not_equal( mach :&mut Machine ,comp1: U4,comp2 : U4, dest: U4){
                if mach.registers[comp1] != mach.registers[comp2]{
                        reg_jump(mach, dest.0 , U4::new(0));
                }

            }
            pub fn reg_shift_left( mach :&mut Machine ,dest: U4,source : U4, shift_amount : U4){
                let mut val = mach.registers[source];
                val = val << (shift_amount.0 as u16) ;
                mach.registers[dest] = val;
            }
            pub fn reg_shift_right( mach :&mut Machine ,dest: U4,source : U4,shift_amount : U4){
                let mut val = mach.registers[source];
                val = val >> (shift_amount.0 as u16) ;
                mach.registers[dest] = val;
            }



            // I- FORMAT
            pub fn reg_immediate_load_word(mach :&mut Machine ,dest: U4,_input :  u8){
                        mach.registers[dest] = _input as u16;


            }
            pub fn reg_immediate_store_word(mach :&mut Machine ,dest: u8 ,_input : U4  ){
                        mach.memory.write(dest as u16, _input.0);
                
            }
            pub fn reg_immediate_add(mach :&mut Machine ,dest: U4,source : U4,_input :  U4){
                mach.registers[dest] = mach.registers[source] + _input.0 as u16;

            }
            pub fn reg_immediate_sub(mach :&mut Machine ,dest: U4,source : U4,_input :  U4){
                mach.registers[dest] = mach.registers[source] - _input.0 as u16;

            }
            pub fn reg_immediate_and(mach :&mut Machine ,dest: U4,source : U4,_input :  U4){
                mach.registers[dest] = mach.registers[source] & _input.0 as u16;

            }
            pub fn reg_immediate_or(mach :&mut Machine ,dest: U4,source : U4,_input :  U4){
                mach.registers[dest] = mach.registers[source] | _input.0 as u16;

            }

            // J - FORMAT
            pub fn reg_jump(mach :&mut Machine ,dest: u8, offset : U4){
                mach.registers[Register::PC as usize] = ( dest *  (offset.0  +1)) as u16;
            }