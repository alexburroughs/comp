use super::data;

struct System<'a> {
    num_stack : &'a mut data::NumStack
}

impl<'a> System<'a> {

    fn new(&mut self, stack : &'a mut data::NumStack) {
        self.num_stack = stack;
    }

    fn f_run(&mut self, args : Vec<String>) {

    }
}