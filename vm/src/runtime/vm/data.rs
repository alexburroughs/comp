pub enum ValueType {
    NUM(f32),
    STR(String),
    LIST(Vec<ValueType>)
}

pub struct NumStack {
    val : Vec<f32>,
}

pub struct ScopeStack {
    val : Vec<usize>
}

pub struct MemStack {
    val : Vec<ValueType>,
    bos : usize,
    arg_count : usize
}

impl Clone for ValueType {
    fn clone(&self) -> ValueType {
        match self {
            ValueType::NUM(val) => {ValueType::NUM(val.clone())},
            ValueType::STR(val) => {ValueType::STR(val.clone())},
            ValueType::LIST(val) => {ValueType::LIST(val.to_vec())}
        }
    }
}

impl NumStack {
    pub fn new() -> NumStack {
        NumStack {
            val : Vec::new()
        } 
    }

    pub fn add(&mut self) {
        let first = self.val.pop().expect("Error: empty stack");
        let second = self.val.pop().expect("Error: empty stack");   

        self.val.push(first + second);
    }

    pub fn sub(&mut self) {
        let first = self.val.pop().expect("Error: empty stack");
        let second = self.val.pop().expect("Error: empty stack");   

        self.val.push(second - first);
    }

    pub fn div(&mut self) {
        let first = self.val.pop().expect("Error: empty stack");
        let second = self.val.pop().expect("Error: empty stack");   

        self.val.push(second / first);
    }

    pub fn mul(&mut self) {
        let first = self.val.pop().expect("Error: empty stack");
        let second = self.val.pop().expect("Error: empty stack");   

        self.val.push(first * second);
    }

    pub fn modulus(&mut self) {
        let first = self.val.pop().expect("Error: empty stack");
        let second = self.val.pop().expect("Error: empty stack");   

        self.val.push(second % first);
    }

    pub fn cmp(&mut self) {
        let first = self.val.pop().expect("Error: empty stack");
        let second = self.val.pop().expect("Error: empty stack");

        self.val.push(if second == first {1.0} else {0.0}); 
    }

    pub fn cmpg(&mut self) {
        let first = self.val.pop().expect("Error: empty stack");
        let second = self.val.pop().expect("Error: empty stack");
        
        self.val.push(if second > first {1.0} else {0.0}); 
    }

    pub fn cmpl(&mut self) {
        let first = self.val.pop().expect("Error: empty stack");
        let second = self.val.pop().expect("Error: empty stack");
        
        self.val.push(if second < first {1.0} else {0.0}); 

    }

    pub fn not(&mut self) {
        let first = self.val.pop().expect("Error: empty stack");
        
        self.val.push(if first == 1.0 {0.0} else {1.0}); 
    }

    pub fn and(&mut self) {
        let first = self.val.pop().expect("Error: empty stack");
        let second = self.val.pop().expect("Error: empty stack");
        
        self.val.push(if second == 1.0 && first == 1.0 {1.0} else {0.0});
    }

    pub fn or(&mut self) {
        let first = self.val.pop().expect("Error: empty stack");
        let second = self.val.pop().expect("Error: empty stack");
        
        self.val.push(if second == 1.0 || first == 1.0 {1.0} else {0.0});
    } 

    pub fn xor(&mut self) {
        let first = self.val.pop().expect("Error: empty stack");
        let second = self.val.pop().expect("Error: empty stack");
        
        self.val.push(if (second == 1.0 && first == 0.0) || (second == 0.0 && first == 1.0) {1.0} else {0.0});
    }

    pub fn ifeq(&mut self) -> bool {
        let first = self.val.pop().expect("Error: empty stack");

        first == 1.0
    }

    pub fn push(&mut self, mem_stack : &MemStack, ind : usize) {
        let entry = match mem_stack.val[ind + mem_stack.bos] {
            ValueType::NUM(var) => {var},
            _ => {panic!("")}
        };

        self.val.push(entry);
    }

    pub fn pop(&mut self, mem_stack : &mut MemStack, ind : usize) {
        
        let entry = self.val.pop().expect("Error: empty stack");

        mem_stack.val[ind + mem_stack.bos] = ValueType::NUM(entry);
    }
}

impl ScopeStack {
    pub fn new() -> ScopeStack {
        ScopeStack {
            val : Vec::new()
        } 
    }

    pub fn push(&mut self, mem_stack : &mut MemStack) {
        self.val.push(mem_stack.bos);
        mem_stack.bos = mem_stack.val.len() - mem_stack.arg_count;
        mem_stack.arg_count = 0;
    }

    pub fn pop(&mut self, mem_stack : &mut MemStack) {
        let pop_to = mem_stack.bos;
        mem_stack.bos = self.val.pop().expect("Error: Can't pop empty scope stack");
        while mem_stack.val.len() < pop_to {
            mem_stack.val.pop();
        }
    }
}

impl MemStack {
    pub fn new() -> MemStack {
        MemStack {
            val : Vec::new(),
            bos : 0,
            arg_count : 0
        }
    }

    pub fn push(&mut self, v_type : ValueType) {
        self.val.push(v_type);
    }

    pub fn set(&mut self, ind : usize, val : ValueType) {
        self.val.get(ind).expect("Error: Memory pointer doesn't exist");
        self.val[ind + self.bos] = val;
    }

    pub fn push_arg(&mut self, ind : usize) {
        self.push(self.val.get(ind + self.bos).expect("Error: Memory pointer to param doesn't exist").clone());
        self.arg_count += 1;
    }

    pub fn get_ret(&self, ind : usize) -> ValueType {
        return self.val.get(ind).expect("Error: Invalid return memory pointer").clone();
    }

    pub fn get(&self, ind : usize) -> ValueType {
        self.val.get(ind + self.bos).expect("Error: Can't retrive pointer").clone()
    }
}