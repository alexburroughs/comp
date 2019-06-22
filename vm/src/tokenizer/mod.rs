pub mod command;

pub fn tokenize(in_str : &String) -> Vec<command::command> {

    let split_str = in_str.split("\n").collect::<Vec<_>>();
    let mut commands : Vec<command::command> = Vec::new();

    for curr_str in split_str {

        let mut com_args = curr_str.split(" ").collect::<Vec<_>>();
        let mut arg_list : Vec<String> = Vec::new();

        let curr_com_type : command::commandType;

        match com_args[0] {
            
            "FS" => {curr_com_type = command::commandType::FS},
            "FE" => {curr_com_type = command::commandType::FE},
            "NEW" => {curr_com_type = command::commandType::NEW},
            "SET" => {curr_com_type = command::commandType::SET},
            "PUSH" => {curr_com_type = command::commandType::PUSH},
            "POP" => {curr_com_type = command::commandType::POP},
            "ADD" => {curr_com_type = command::commandType::ADD},
            "SUB" => {curr_com_type = command::commandType::SUB},
            "MUL" => {curr_com_type = command::commandType::MUL},
            "DIV" => {curr_com_type = command::commandType::DIV},
            "MOD" => {curr_com_type = command::commandType::MOD},
            "CMP" => {curr_com_type = command::commandType::CMP},
            "CMPG" => {curr_com_type = command::commandType::CMPG},
            "CMPL" => {curr_com_type = command::commandType::CMPL},
            "NOT" => {curr_com_type = command::commandType::NOT},
            "AND" => {curr_com_type = command::commandType::AND},
            "OR" => {curr_com_type = command::commandType::OR},
            "XOR" => {curr_com_type = command::commandType::XOR},
            "IFEQ" => {curr_com_type = command::commandType::IFEQ},
            "JMP" => {curr_com_type = command::commandType::JMP},
            "CALL" => {curr_com_type = command::commandType::CALL},
            _ => {continue}
        }

        com_args.remove(0);

        for arg in com_args {
            let tmp : String = String::from(arg);
            arg_list.push(tmp.clone());
        }

        let comm = command::command {
            c_type : curr_com_type,
            args : arg_list
        };
    
        commands.push(comm);
    } 

    return commands;
}