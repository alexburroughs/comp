use super::data;

struct System<'a> {
    num_stack : &'a mut data::NumStack
}

impl<'a> System<'a> {

    fn new( stack : &'a mut data::NumStack) -> System<'a>{
        System {
            num_stack : stack
        }
    }

    fn f_run(&mut self, args : Vec<String>) {

    }
}