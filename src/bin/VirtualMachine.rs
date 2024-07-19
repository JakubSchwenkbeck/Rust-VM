use operating_system::user_interface::cmd_line_interface;
use rust_projects::*;


pub fn main()-> Result<(), & 'static str> {
    let mut virtualm = Machine::new();  
    
    cmd_line_interface(&mut virtualm);

    virtualm.step()
}
