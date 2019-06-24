pub enum ValueType {
    NUM(f32),
    STR(String),
    LIST(Vec<ValueType>)
}

pub struct NumStack {
    val : Vec<i32>
}

pub struct ScopeStack {
    val : Vec<usize>
}

pub struct MemStack {
    val : Vec<ValueType>,
    bos : i32
}

impl NumStack {
    pub fn new() -> NumStack {
        NumStack {
            val : Vec::new()
        } 
    }
}

impl ScopeStack {
    pub fn new() -> ScopeStack {
        ScopeStack {
            val : Vec::new()
        } 
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