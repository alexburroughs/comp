use std::collections::HashMap;
use super::command;
//use std::string::ToString;

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

    pub fn run(&mut self, commands : Vec<command::Command>, f_map : HashMap<String, (usize, usize)>, a_map : HashMap<String, usize>) {

        let mut x = 0;

        // loop until commands are finished
        while x != commands.len() {

            let com = &commands[x];

            //println!("{} : {}", x, com.c_type.to_string());

            // match on the command type
            match com.c_type {
                command::CommandType::FS => {
                    let tmp = (f_map
                        .get(com.args[0]
                        .as_str()))
                        .expect("error unwrapping function")
                        .clone();

                    x = tmp.1;
                },
                command::CommandType::FE => {
                    match com.args.get(1) {
                        Some(val) => {self.ret = Some(self.mem_stack.get_ret(val.parse().expect("Error: Invalid return argument")))},
                        None => {self.ret = None}
                    }

                    x = self.scope_stack.pop(&mut self.mem_stack);
                },
                command::CommandType::NEW => {
                    self.mem_stack.push(match com.args
                        .get(0)
                        .expect("Error: Invalid number of arguments to new")
                        .as_str() {
                            "NUM" => {ValueType::NUM(0.0)},
                            "STR" => {ValueType::STR(String::from(""))},
                            "LIST" => {ValueType::LIST(Vec::new())},
                            _ => {panic!("Error: Invalid type argument to new")}
                    });
                },
                command::CommandType::RM => {
                    self.mem_stack.pop();
                },
                command::CommandType::SET => {
                    self.mem_stack.set(com.args
                        .get(0)
                        .expect("Error: Invalid number of arguments to set")
                        .parse()
                        .expect("Error: Invalid argument to set"),
                        match com.args.get(1).expect("Error: Invalid arguments to set").as_str() {
                            "NUM" => {
                                ValueType::NUM(com.args
                                    .get(2)
                                    .expect("Error: Invalid number of arguments to set")
                                    .parse()
                                    .expect("Error: Invalid number of arguments to set")
                                )
                            },
                            "STR" => {
                                ValueType::STR(com.args
                                    .get(2)
                                    .expect("Error: Invalid number of arguments to set")
                                    .clone())
                            },
                            "RET" => {
                                self.ret.clone().expect("Error: Return value is null")
                            },
                            _ => {panic!("Error: Invalid type argument to set")}
                        }
                    );
                },
                command::CommandType::COPY => {
                    self.mem_stack.copy(com.args
                        .get(0)
                        .expect("Error: Invalid number of arguments to copy")
                        .parse()
                        .expect("Error: Invalid argument to copy"),
                        com.args
                        .get(1)
                        .expect("Error: Invalid number of arguments to copy")
                        .parse()
                        .expect("Error: Invalid argument to copy")
                    );
                },
                command::CommandType::PUSH => {

                    self.num_stack.push(&self.mem_stack, com.args
                        .get(0)
                        .expect("Error: Invalid number of arguments to push")
                        .parse()
                        .expect("Error: Invalid argument to push"))
                },
                command::CommandType::POP => {
                    self.num_stack.pop(&mut self.mem_stack, com.args
                        .get(0)
                        .expect("Error: Invalid number of arguments to pop")
                        .parse()
                        .expect("Error: Invalid argument to pop"))
                },
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
                        x = a_map.get(&com.args[0]).expect("Error: invalid address").clone();
                    }
                },
                command::CommandType::JMP => {
                    x = a_map.get(&com.args[0]).expect("Error: invalid address").clone();
                },
                command::CommandType::SYS => {
                    let mut sys = system::System::new(&mut self.mem_stack, &mut self.num_stack);
                    sys.f_run(&com.args);
                },
                command::CommandType::CALL => {
                    let last_x = x;
                    let tmp = (f_map
                        .get(com.args[0]
                        .as_str()))
                        .expect("error unwrapping function")
                        .clone();

                    x = tmp.0;
                    for y in 1..com.args.len() {
                        self.mem_stack.push_arg(com.args
                                                   .get(y)
                                                   .expect("Error: can't unwrap call argument")
                                                   .parse()
                                                   .expect("Error: Invalid argument to call"));
                    }

                    self.scope_stack.push(&mut self.mem_stack, last_x);
                },
                command::CommandType::LS_ADD => {
                    self.mem_stack.ls_add(
                        com.args.get(0)
                            .expect("Error: Invalid number of arguments to ls_add")
                            .parse()
                            .expect("Error: Invalid argument to ls_add"),
                        com.args.get(1)
                            .expect("Error: Invalid number of arguments to ls_add")
                            .to_string()
                    );

                },
                command::CommandType::LS_GET => {
                    self.mem_stack.ls_get(
                        com.args.get(0)
                            .expect("Error: Invalid number of arguments to ls_get")
                            .parse()
                            .expect("Error: Invalid argument to ls_get"),
                        com.args.get(1)
                            .expect("Error: Invalid number of arguments to ls_get")
                            .parse()
                            .expect("Error: Invalid argument to ls_get"),
                        com.args.get(2)
                            .expect("Error: Invalid number of arguments to ls_get")
                            .parse()
                            .expect("Error: Invalid argument to ls_get"),

                    )
                },
                command::CommandType::LS_RM => {
                    self.mem_stack.ls_rm(
                        com.args.get(0)
                            .expect("Error: Invalid number of arguments to ls_get")
                            .parse()
                            .expect("Error: Invalid argument to ls_get"),
                        com.args.get(1)
                            .expect("Error: Invalid number of arguments to ls_get")
                            .parse()
                            .expect("Error: Invalid argument to ls_get")
                    );
                },
                command::CommandType::LS_SIZE => {
                    self.mem_stack.ls_size(
                        com.args.get(0)
                            .expect("Error: Invalid number of arguments to ls_size")
                            .parse()
                            .expect("Error: Invalid argument to ls_size"),
                        com.args.get(1)
                            .expect("Error: Invalid number of arguments to ls_size")
                            .parse()
                            .expect("Error: Invalid argument to ls_size")
                    )
                },
                command::CommandType::RET => {},
                command::CommandType::ADDR => {}

            }

            x+=1;
        }
    }
}