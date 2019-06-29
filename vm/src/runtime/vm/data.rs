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
    bos : usize
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

    pub fn push(mem_stack : &mut MemStack) {

    }

    pub fn pop(mem_stack : &mut MemStack) {

    }
}

impl MemStack {
    pub fn new() -> MemStack {
        MemStack {
            val : Vec::new(),
            bos : 0
        }
    }

    pub fn push(&mut self, v_type : ValueType) {
        self.val.push(v_type);
    }

    pub fn set(&mut self, ind : usize, val : ValueType) {
        self.val.get(ind).expect("Memory pointer doesn't exist");
        self.val[ind] = val;
    }
}