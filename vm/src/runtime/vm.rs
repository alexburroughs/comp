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

    pub fn run(&mut self, commands : Vec<command::Command>, f_map : HashMap<&str, (usize, usize)>, a_map : HashMap<&str, usize>) {

        let mut x = 0;

        // loop until commands are finished
        while x != commands.len() {

            let com = &commands[x];

            match com.c_type {
                command::CommandType::FS => {
                    let tmp = (f_map.get(com.args[0].as_str())).expect("error unwrapping function").clone();
                    x = tmp.1;
                },
                command::CommandType::FE => {},
                command::CommandType::NEW => {

                },
                command::CommandType::SET => {},
                command::CommandType::PUSH => {},
                command::CommandType::POP => {},
                command::CommandType::ADD => {
                    self.num_stack.add();
                },
                command::CommandType::SUB => {
                    self.num_stack.sub();
                },
                command::CommandType::MUL => {
                    self.num_stack.mul();
                },
                command::CommandType::DIV => {
                    self.num_stack.div();
                },
                command::CommandType::MOD => {
                    self.num_stack.modulus();
                },
                command::CommandType::CMP => {
                    self.num_stack.cmp();
                },
                command::CommandType::CMPG => {
                    self.num_stack.cmpg();
                },
                command::CommandType::CMPL => {
                    self.num_stack.cmpl();
                },
                command::CommandType::NOT => {
                    self.num_stack.not();
                },
                command::CommandType::AND => {
                    self.num_stack.and();
                },
                command::CommandType::OR => {
                    self.num_stack.or();
                },
                command::CommandType::XOR => {
                    self.num_stack.xor();
                },
                command::CommandType::IFEQ => {
                    if self.num_stack.ifeq() {
                        x = a_map.get(&com.args[0].as_str()).expect("Error: invalid address").clone();
                    }
                },
                command::CommandType::JMP => {
                    x = a_map.get(&com.args[0].as_str()).expect("Error: invalid address").clone();
                },
                command::CommandType::SYS => {},
                command::CommandType::CALL => {},
                command::CommandType::ADDR => {}
            }
        }
    }
}