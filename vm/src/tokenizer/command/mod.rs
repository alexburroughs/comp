pub struct command {
    pub c_type : commandType,
    pub args : Vec<String>
}

pub enum commandType {
    FS,
    FE,
    NEW,
    SET,
    PUSH,
    POP,
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
    CMP,
    CMPG,
    CMPL,
    NOT,
    AND,
    OR,
    XOR,
    IFEQ,
    JMP,
    CALL
}