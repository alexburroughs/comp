pub enum ValueType {
    NUM(f32),
    STR(String),
    LIST(Vec<ValueType>)
}

pub struct NumStack {
    val : Vec<f32>,
}

pub struct ScopeStack {
    val : Vec<usize>,
    ins : Vec<usize>
}

pub struct MemStack {
    val : Vec<ValueType>,
    bos : usize,
    arg_count : usize
}

const NUM_IDENT : char = 'N';
const STRING_IDENT : char = 'S';

fn from_num_literal(mut literal : String) -> f32 {
    literal.remove(0);
    return literal.parse().expect("Error: Invalid literal");
}

fn from_str_literal(mut literal : String) -> String {
    literal.remove(0);
    return literal;
}

fn is_literal(literal : &mut String) -> bool {
    match literal.chars().next().expect("Error: Empty argument") {
        NUM_IDENT => true,
        STRING_IDENT => true,
        _ => false
    }
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
            _ => {panic!("Error: Invalid type to push")}
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
            val : Vec::new(),
            ins : Vec::new()
        } 
    }

    pub fn push(&mut self, mem_stack : &mut MemStack, instruction : usize) {
        self.val.push(mem_stack.bos);
        mem_stack.bos = mem_stack.val.len() - mem_stack.arg_count;
        mem_stack.arg_count = 0;
        self.ins.push(instruction);
    }

    pub fn pop(&mut self, mem_stack : &mut MemStack) -> usize {
        let pop_to = mem_stack.bos;
        mem_stack.bos = self.val.pop().expect("Error: Can't pop empty scope stack");
        while mem_stack.val.len() > pop_to {
            mem_stack.val.pop();

        }
        self.ins.pop().expect("Error: Empty instruction in scope stack")
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

    pub fn get_bos(&self) -> usize {
        return self.bos;
    }

    pub fn push(&mut self, v_type : ValueType) {
        self.val.push(v_type);
    }

    pub fn pop(&mut self) {
        self.val.pop().expect("Error: empty stack");
    }

    pub fn set(&mut self, ind : usize, val : ValueType) {
        self.val.get(ind + self.bos).expect("Error: Memory pointer doesn't exist");
        self.val[ind + self.bos] = val;
    }

    pub fn copy(&mut self, dest : usize, src : usize) {
        self.val.get(src + self.bos).expect("Error: Memory pointer doesn't exist");
        self.val.get(dest + self.bos).expect("Error: Memory pointer doesn't exist");
        self.val[dest + self.bos] = self.val[src + self.bos].clone();
    }

    pub fn ls_add(&mut self, ind : usize, val : String) {

        let tmp : ValueType;

        if is_literal(&mut val.clone()) {
            match val.clone().chars().clone().next().expect("Error: Empty argument").clone() {
                NUM_IDENT => {tmp = ValueType::NUM(from_num_literal(val))},
                STRING_IDENT => {tmp = ValueType::STR(from_str_literal(val))},
                _ => {panic!("Error: Unexpected error occured")}
            }
        }
        else {
            self.val.get(ind + self.bos).expect("Error: Memory pointer doesn't exist");
            tmp = self.val[val.parse::<usize>().expect("Error: Invalid pointer passed to ls_add") + self.bos].clone();
        }

        match &mut self.val[ind + self.bos] {
            ValueType::LIST(v) => {v.push(tmp)},
            _ => {panic!("Error: Non list passed to list function ls_add")}
        }
    }

    pub fn ls_get(&mut self, ind : usize, src : usize, dest : usize) {
 
        self.val.get(ind + self.bos).expect("Error: Memory pointer doesn't exist");
        let dest_val = self.val.get(dest + self.bos).expect("Error: Destination pointer doesn't exist").clone();
        let src_val = match self.val.get(src + self.bos).expect("Error: Destination pointer doesn't exist").clone() {
            ValueType::NUM(val) => {val},
            _ => {panic!("Index doesn't exist")}
        };

        match &mut self.val[ind + self.bos] {
            ValueType::LIST(v) => {
                let tmp = v.get(src_val as usize).expect("Error: Source pointer doesn't exist");
                if std::mem::discriminant(tmp) == std::mem::discriminant(&dest_val) {
                    self.val[dest + self.bos] = tmp.clone();
                }
                else {
                    panic!("Error: Mismatched types source and destination")
                }
            },
            _ => {panic!("Error: Non list passed to list function ls_get")}
        }
    }

    pub fn ls_rm(&mut self, ind : usize, src : usize) {
        
        self.val.get(ind + self.bos).expect("Error: Memory pointer doesn't exist");
        
        match &mut self.val[ind + self.bos] {
            ValueType::LIST(v) => {
                v.remove(src);
            },
            _ => {panic!("Error: Non list passed to list function ls_get")}
        }
    }

    pub fn ls_size(&mut self, ind : usize, dest : usize) {
        self.val.get(ind + self.bos).expect("Error: Memory pointer doesn't exist");
        self.val.get(dest + self.bos).expect("Error: Destination pointer doesn't exist");
        match &mut self.val[ind + self.bos] {
            ValueType::LIST(v) => {
                self.val[dest + self.bos] = ValueType::NUM(v.len() as f32);
            },
            _ => {panic!("Error: Non list passed to list function ls_get")}

        }
    }

    pub fn push_arg(&mut self, ind : usize) {
        self.push(self.val.get(ind + self.bos).expect("Error: Memory pointer to param doesn't exist").clone());
        self.arg_count += 1;
    }

    pub fn get_ret(&self, ind : usize) -> ValueType {
        return self.val.get(ind + self.bos).expect("Error: Invalid return memory pointer").clone();
    }

    pub fn get(&self, ind : usize) -> ValueType {
        self.val.get(ind + self.bos).expect("Error: Can't retrieve pointer").clone()
    }
}