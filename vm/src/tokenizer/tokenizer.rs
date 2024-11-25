use super::super::runtime::command;
use std::collections::HashMap;
use std::str;

fn split(str : &str, splitter : char) -> Vec<String>{
    let mut escape = false;
    let mut in_str = false;
    let mut ret = Vec::new();
    let mut current_str = String::new();
    for c in str.chars() {
        if escape == false && c == '\\' {
            escape = true;
            continue;
        }
        if c == '\"' && escape == false {
            in_str = !in_str;
            continue;
        }

        if !in_str && c == splitter {
            ret.push(current_str.clone());
            current_str = String::new();
        }
        else {
            current_str.push(c)
        }

        escape = false;
    }
    ret.push(current_str.clone());
    return ret;
}

pub fn tokenize(in_str : &String) -> (Vec<command::Command>, HashMap<String, (usize,usize)>, HashMap<String, usize>)  {

    // seperate the code into commands
    let split_str = in_str.split("\n").collect::<Vec<_>>();

    // final list of commands and function map
    let mut commands : Vec<command::Command> = Vec::new();
    let mut f_list : HashMap<String, (usize, usize)> = HashMap::new();
    let mut a_list : HashMap<String, usize> = HashMap::new();

    // loop through each line
    for curr_str in split_str {

        // seperate command and arguments
        let mut com_args = split(curr_str, ' ');
        let mut arg_list : Vec<String> = Vec::new();

        let curr_com_type : command::CommandType;

        //println!("{}\n", com_args[0]);

        // map strings to commands
        match com_args[0].as_str() {
            
            // add function start to the function map
            "FS" => {
                curr_com_type = command::CommandType::FS;

                let mut i : (usize, usize);

                match f_list.get(&com_args[1]) {
                    Some(val) => {i = *val},
                    None => {i = (0,0)}
                }

                i.0 = commands.len();
                f_list.insert(com_args[1].clone(), i);
            },

            // add function end to the function map
            "FE" => {
                curr_com_type = command::CommandType::FE;

                let mut i : (usize, usize);
               
                match f_list.get(&com_args[1]) {
                    Some(val) => {i = *val},
                    None => {i = (0,0)}
                }
                
                i.1 = commands.len();
                f_list.insert(com_args[1].clone(), i);
                
                },
            "NEW" => {curr_com_type = command::CommandType::NEW},
            "RM" => {curr_com_type = command::CommandType::RM},
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
            "COPY" => {curr_com_type = command::CommandType::COPY},
            "ADDR" => {             
                curr_com_type = command::CommandType::ADDR;
                let tmp = commands.len();
                a_list.insert(com_args[1].clone(), tmp);
            },
            "RET" => {curr_com_type = command::CommandType::RET},
            "LS_ADD" => {curr_com_type = command::CommandType::LS_ADD},
            "LS_GET" => {curr_com_type = command::CommandType::LS_GET},
            "LS_RM" => {curr_com_type = command::CommandType::LS_RM},
            "" => {continue}
            _ => {
                    if com_args[0].starts_with('#') {
                        continue
                    }
                    panic!("Error: Invalid command \"{}\" ", com_args[0]);
                }
        }

        // remove the command, keep the arguments
        com_args.remove(0);

        // add the arguments to the command object
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

    return (commands, f_list, a_list);
}