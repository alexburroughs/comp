use super::data;

pub struct System<'a> {
    mem_stack : &'a mut data::MemStack
}

impl<'a> System<'a> {

    pub fn new( stack : &'a mut data::MemStack) -> System<'a>{
        System {
            mem_stack : stack
        }
    }

    pub fn f_run(&mut self, args : &Vec<String>) {

        match args.get(0).expect("Error: Invalid amount of arguments to system call").as_str() {
            "PRINT" => {self.print(args.get(1).expect("Error: Invalid number of arguements to print").parse().expect("Error: Invalid argument to print"))},
            _ => {panic!("Error: Invalid system call")}
        }
    }

    fn print(&mut self, ind : usize) {
        let tmp = self.mem_stack.get(ind);

        match tmp {
            data::ValueType::NUM(val) => {println!("{}", val)},
            data::ValueType::STR(val) => {println!("{}", val)},
            _ => {}
        }

    }   
}