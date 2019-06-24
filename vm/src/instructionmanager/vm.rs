use std::collections::HashMap;
use super::command;

mod data;
mod system;

use data::*;

pub struct Vm {

    mem_stack : MemStack,
    scope_stack : ScopeStack,
    num_stack : NumStack,
    ret : Option<ValueType>
}

impl Vm {

    pub fn new() -> Vm {
        Vm {
            mem_stack : MemStack::new(),
            scope_stack : ScopeStack::new(),
            num_stack : NumStack::new(),
            ret : None
        }
    }

    pub fn run(&mut self, commands : Vec<command::Command>, f_map : HashMap<&str, (usize, usize)>) {

        let mut x = 0;

        // loop until commands are finished
        while x != commands.len() {

            let com = &commands[x];

            match com.c_type {
                command::CommandType::FS => {
                    let tmp = *((f_map.get(com.args[0].as_str())).expect("error unwrapping function"));
                    x = tmp.1;
                },
                command::CommandType::FE => {},
                command::CommandType::NEW => {

                },
                command::CommandType::SET => {},
                command::CommandType::PUSH => {},
                command::CommandType::POP => {},
                command::CommandType::ADD => {},
                command::CommandType::SUB => {},
                command::CommandType::MUL => {},
                command::CommandType::DIV => {},
                command::CommandType::MOD => {},
                command::CommandType::CMP => {},
                command::CommandType::CMPG => {},
                command::CommandType::CMPL => {},
                command::CommandType::NOT => {},
                command::CommandType::AND => {},
                command::CommandType::OR => {},
                command::CommandType::XOR => {},
                command::CommandType::IFEQ => {},
                command::CommandType::JMP => {},
                command::CommandType::SYS => {},
                command::CommandType::CALL => {},
            }
        }
    }
}