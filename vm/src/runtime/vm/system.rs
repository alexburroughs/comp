use super::data;

use std::io;

pub struct System<'a> {
    mem_stack : &'a mut data::MemStack,
    num_stack : &'a mut data::NumStack
}

impl<'a> System<'a> {

    pub fn new( m_stack : &'a mut data::MemStack, n_stack : &'a mut data::NumStack) -> System<'a>{
        System {
            mem_stack : m_stack,
            num_stack : n_stack
        }
    }

    pub fn f_run(&mut self, args : &Vec<String>) {
        //println!("{}", args[0]);
        match args.get(0)
                .expect("Error: Invalid amount of arguments to system call")
                .as_str() {

            "PRINT" => {
                self.f_print(args.get(1)
                    .expect("Error: Invalid number of arguements to print")
                    .parse()
                    .expect("Error: Invalid argument to print"))},
            "IN" => { 
                self.f_in(args.get(1)
                    .expect("Error: Invalid number of arguements to in")
                    .parse()
                    .expect("Error: Invalid argument to in"))},
            _ => {panic!("Error: Invalid system call")}
        }
    }

    fn f_print(&mut self, ind : usize) {
        let tmp = self.mem_stack.get(ind);

        match tmp {
            data::ValueType::NUM(val) => {println!("{}", val)},
            data::ValueType::STR(val) => {println!("{}", val)},
            _ => {}
        }
    }   

    fn f_in(&mut self, ind : usize) {

        let mut inp = String::new();
        
        io::stdin().read_line(&mut inp)
        .ok()
        .expect("Couldn't read line");

        inp = String::from(inp.trim_end());
        let tmp = self.mem_stack.get(ind);

        match tmp {
            data::ValueType::NUM(_) => {self.mem_stack.set(ind, data::ValueType::NUM(inp.parse().expect("Error: Expected num, got str for in")));},
            data::ValueType::STR(_) => {self.mem_stack.set(ind,data::ValueType::STR(inp));},
            _ => {panic!("Error: Can't input to list")}
        }
    }
}