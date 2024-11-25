use std::fmt;

pub struct Command {
    pub c_type : CommandType,
    pub args : Vec<String>
}

#[derive(Debug)]
pub enum CommandType {
    FS,
    FE,
    NEW,
    RM,
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
    SYS,
    CALL,
    ADDR,
    RET,
    LS_ADD,
    LS_GET,
    LS_RM,
    LS_SIZE,
    COPY
}

impl fmt::Display for CommandType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}