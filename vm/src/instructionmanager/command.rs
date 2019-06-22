pub struct Command {
    pub c_type : CommandType,
    pub args : Vec<String>
}

pub enum CommandType {
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