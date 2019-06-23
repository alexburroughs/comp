use std::collections::HashMap;
use super::command;

mod data;
mod system;

pub fn run(commands : Vec<command::Command>, f_map : HashMap<&str, (usize, usize)>) {

    let mem_stack : data::MemStack;
    let scope_stack : data::ScopeStack;
    let num_stack : data::NumStack;

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
            command::CommandType::NEW => {},
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