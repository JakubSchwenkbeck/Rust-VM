





use instructions::instructions_regs;
use instructions_regs::*;
use interpreter::assembler::*;
use interpreter::decoder::decode;
// Virtual Machine Binary 
use rust_projects::*;


pub fn main()-> Result<(), & 'static str> {
    let mut virtualm = Machine::new();
  
    println!("--------------------------------------");
    parse_programm(&mut virtualm);

    


    for i in 0..10{

        let val = virtualm.memory.read2(i*2).unwrap();
        let v =  format!("{:016b}", val);
        println!("Memory space  {i} : {val} with binary {v}");
    
    };

    
    run_programm(&mut virtualm);

    reg_printall(&mut virtualm);

    let p = virtualm.registers[7];
    println!("Fibonacci : {p}" );
    virtualm.step()
    


}

pub fn run_programm(virtualm :&mut Machine){
  
        let mut l = virtualm.memory.read2(0 as u16).unwrap();
        while l != 0 {
            decode(l as u16, virtualm);
           
            virtualm.registers[13] += 2; 
             
             l = virtualm.memory.read2(  virtualm.registers[13]as u16).unwrap();
             println!("{l}");
             let _ =  virtualm.step();
             reg_printall( virtualm);

        }


}


pub fn parse_programm(virtualm :&mut Machine){
    let lines =read_lines_from_file("Assembly.txt").unwrap();
    let mut index= 0;
    for line in lines{
        /*parse_line(&line, &mut virtualm);
        reg_printall(&mut virtualm);    
        virtualm.step()?;*/

        let val = parse_line(&line,   virtualm);

        virtualm.memory.write2(index,val );

            index += 2;

    }   

}