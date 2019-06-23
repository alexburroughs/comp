use super::command;
use std::collections::HashMap;

pub fn tokenize(in_str : &String) -> (Vec<command::Command>, HashMap<&str, (usize,usize)>)  {

    let split_str = in_str.split("\n").collect::<Vec<_>>();
    let mut commands : Vec<command::Command> = Vec::new();
    let mut f_list : HashMap<&str, (usize, usize)> = HashMap::new();

    for curr_str in split_str {

        let mut com_args = curr_str.split(" ").collect::<Vec<_>>();
        let mut arg_list : Vec<String> = Vec::new();

        let curr_com_type : command::CommandType;

        // map strings to commands
        match com_args[0] {
            "FS" => {
                curr_com_type = command::CommandType::FS;

                let mut i : (usize, usize);

                match f_list.get(com_args[1]) {
                    Some(val) => {i = *val},
                    None => {i = (0,0)}
                }

                i.0 = commands.len() + 1;
                f_list.insert(com_args[1], i);
            },
            "FE" => {
                curr_com_type = command::CommandType::FE;

                let mut i : (usize, usize);

                match f_list.get(com_args[1]) {
                    Some(val) => {i = *val},
                    None => {i = (0,0)}
                }
                
                i.1 = commands.len() + 1;
                f_list.insert(com_args[1], i);
                
                },
            "NEW" => {curr_com_type = command::CommandType::NEW},
            "SET" => {curr_com_type = command::CommandType::SET},
            "PUSH" => {curr_com_type = command::CommandType::PUSH},
            "POP" => {curr_com_type = command::CommandType::POP},
            "ADD" => {curr_com_type = command::CommandType::ADD},
            "SUB" => {curr_com_type = command::CommandType::SUB},
            "MUL" => {curr_com_type = command::CommandType::MUL},
            "DIV" => {curr_com_type = command::CommandType::DIV},
            "MOD" => {curr_com_type = command::CommandType::MOD},
            "CMP" => {curr_com_type = command::CommandType::CMP},
            "CMPG" => {curr_com_type = command::CommandType::CMPG},
            "CMPL" => {curr_com_type = command::CommandType::CMPL},
            "NOT" => {curr_com_type = command::CommandType::NOT},
            "AND" => {curr_com_type = command::CommandType::AND},
            "OR" => {curr_com_type = command::CommandType::OR},
            "XOR" => {curr_com_type = command::CommandType::XOR},
            "IFEQ" => {curr_com_type = command::CommandType::IFEQ},
            "JMP" => {curr_com_type = command::CommandType::JMP},
            "SYS" => {curr_com_type = command::CommandType::SYS},
            "CALL" => {curr_com_type = command::CommandType::CALL},
            _ => {continue}
        }

        // remove the command, keep the arguments
        com_args.remove(0);

        for arg in com_args {
            let tmp : String = String::from(arg);
            arg_list.push(tmp.clone());
        }

        let comm = command::Command {
            c_type : curr_com_type,
            args : arg_list
        };
    
        commands.push(comm);
    } 

    return (commands, f_list);
}