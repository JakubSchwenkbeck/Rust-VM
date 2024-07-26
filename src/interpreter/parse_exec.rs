
/*pub fn run_programm(virtualm :&mut Machine,filename: &str)-> Result<(), & 'static str> {

    virtualm.reset_registers_except_pc();
   let c = get_interval(filename);
   let  start = c.0;
   let end = c.1;
   println!("Start  {start} , End {end}");

   let mut l = virtualm.memory.read2( start).unwrap();
   virtualm.registers[13] = start;
    while l != 0 {
        
         l = virtualm.memory.read2(  virtualm.registers[13]).unwrap();
         decode(l , virtualm,c.0);
         let start =  virtualm.registers[13];
            println!("current : {start}");

         println!("{l}");
        
         reg_printall( virtualm);
         virtualm.registers[13] += 2;
    
    }
        reg_single_print(virtualm, U4::new(15));
    
    Ok(())



}


pub fn parse_programm(virtualm :&mut Machine,filename: &str)-> Result<(), & 'static str> {
let lines =read_lines_from_file(&filename).unwrap();

let mut res: Vec<u16>;
for line in lines{
    res.push(parse_line(&line,   virtualm))



}
let c = get_interval(filename);
let mut start = c.0;
let end = c.1;

for line in lines{
    if start < end{
    /*parse_line(&line, &mut virtualm);
    reg_printall(&mut virtualm);    
    virtualm.step()?;*/

    let val = parse_line(&line,   virtualm);

    virtualm.memory.write2(start,val );
            println!("writing into {start}");
         start += 2;
    }
}   
Ok(())

}
*/