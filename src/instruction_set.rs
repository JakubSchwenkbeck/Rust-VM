
use crate::memory::{LinMem,Addressable};
use crate::vm::Register;

use crate::vm::Machine;


//* The instruction Set contains 17 Instructions which are used for simulating a Processor level control */
//* Is implemented for Registers (as in a CPU) and directly for Linear Memory to give hardware near control in VM using own code */

// Shifting between R,I and jump format

pub enum InstructionSet{

        // Comments for Linear Memory Adressing , Regs have their own format

    Lw, // load Word || Input : <Destination : Addr> <Source : Addr> 
    LwI, // load Word || Input :  <Destination : Addr> <_input : Word>

    SW, // store word ||input : <Destination : Addr>, <_Source : Addr> 
    SWI, // store word ||input : <Destination : Addr>, <_input : Word>
   
    Add, // Add words ||input : <Destination : Addr>, <Source1 : Addr> , <Source2 : Addr>
    AddI, // Add words immediate ||input : <Destination : Addr>,  <Source1 : Addr>, <_input : Word>

    Sub, // sub words ||input : <Destination : Addr>, <Source1 : Addr> , <Source2 : Addr>
    SubI, // sub words immediate |input : <Destination : Addr>,  <Source1 : Addr>, <_input : Word>

    And, //  Logical And || input : <Destination : Addr>, <Source1 : Addr> , <Source2 : Addr>
    AndI, //  Logical And ||input : <Destination : Addr>,  <Source1 : Addr>, <_input : Word>

    Or,//  Logical Or || input : <Destination : Addr>, <Source1 : Addr> , <Source2 : Addr>
    OrI, //  Logical Or ||input : <Destination : Addr>,  <Source1 : Addr>, <_input : Word>

    Jump, //Jump || input : <Destination : Addr>

    Bne, // Branch (if not equal) || <Comp1 : Addr>, <Comp2 : Addr>, <Jump : Addr>

    Slt, // set less than || <comp1 : Addr>, <Comp2 : Addr>, <AddressToSet>

    ShiftLeft, // shift (amount) || <Destination>, <Source :Addr>, <ShiftAmount : Word>
    ShiftRight, // shift (amount) || <Destination>, <Source :Addr>, <ShiftAmount : Word>

    
}
