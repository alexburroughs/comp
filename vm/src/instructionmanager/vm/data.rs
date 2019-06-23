pub enum ValueType {
    NUM,
    STR,
    LIST
}


pub struct Value {
    n_val : i32,
    s_val : String,
    l_val : Vec<Value>,
    v_type : ValueType
}

pub struct NumStack {
    val : Vec<i32>
}

pub struct ScopeStack {
    val : Vec<usize>
}

pub struct MemStack {
    val : Vec<Value>
}