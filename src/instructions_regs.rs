
use crate::memory::{LinMem,Addressable};
use crate::vm::Register;

use crate::vm::Machine;

// Implementing our ISA for Register control on assembler level, mimicking a cpu in mips style

// building for 16 Registers:

// Formats : R - Type, I - Type and J - Type
// Bit use : Read first 2 bytes to determine Format
// --> R - Type  4 bits OpCode, 4 bit address, 4 bit address, 4 Bit Adress XXXX-XXXX-XXXX-XXXX

// --> I - Type 3 bits Opcode, 4 bits adress, 4 bits adress, 5 bits immediate value

// --> J - Type : 16 bits Address

