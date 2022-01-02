use super::super::ast::ast as SyntaxTree;
use std::collections::HashMap;

pub fn generate_code(tree : &mut SyntaxTree::AST) -> String {
    let mut ret = String::from("");

    for function in tree.functions.clone() {
        let mut vars : HashMap<String, (u32, SyntaxTree::ValueType)> = HashMap::new();
        let mut var_num : u32 = 0;
        let mut ret_val : u32 = 0;

        ret.push_str(&format!("FS {}\n", function.name.expect("Error: Can't get function name"))); 

        ret.push_str(parse_children(&function.children, &mut vars, &mut var_num, &mut ret_val).as_str());

        ret.push_str(&format!("FE {}\n", ret_val));
    }

    ret
}

fn parse_children(children : &Vec<SyntaxTree::Value>,
    vars : &mut HashMap<String, (u32, SyntaxTree::ValueType)>,
    var_num : &mut u32, 
    ret_val : &mut u32) -> String {

    let mut ret = String::from("");

    for stmt in children.clone() {
            match stmt.v_type {
                SyntaxTree::ValueType::ARGUMENT => {
                    for arg in stmt.children.clone() {
                        vars.insert(arg.name.expect("Error: Can't get arguments name"), (*var_num, arg.v_type));
                        *var_num += 1;
                    }
                    *ret_val = *var_num;
                    *var_num+=1;
                },
                SyntaxTree::ValueType::DECLARATION => {
                    for x in stmt.children {
                        ret.push_str(&format!("NEW {}\n", match x.v_type {
                            SyntaxTree::ValueType::NUM => {"NUM"},
                            SyntaxTree::ValueType::STR => {"STR"},
                            SyntaxTree::ValueType::LIST => {"LIST"},
                            _ => {panic!("Error: Invalid declaration type")}
                        }))
                    }
                },
                SyntaxTree::ValueType::ASSIGNMENT => {

                    match vars.get(&stmt.name.clone().expect("Error: Can't unwrap variable name")).unwrap().1 {
                        SyntaxTree::ValueType::NUM => {
                            ret.push_str(parse_expression(&stmt.clone().children, vars, var_num, ret_val).as_str());
                        },
                        _ => {panic!("Error: only numbers are supported")}
                    }   

                    ret.push_str(&format!("SET {} {} {}\n",
                        vars.get(&stmt.name.clone().expect("Error: Can't unwrap variable name")).unwrap().0,
                        match vars.get(&stmt.name.clone().expect("Error: Can't unwrap variable name")).unwrap().1 {
                            SyntaxTree::ValueType::NUM => {"NUM"},
                            SyntaxTree::ValueType::STR => {"STR"},
                            SyntaxTree::ValueType::LIST => {"LIST"},
                            _ => {panic!("Error: Invalid declaration type")}
                        },
                        *var_num)
                    );

                    *var_num+=1;
                }
                _ => {}
            }
        }

    ret
}

fn parse_expression(children : &Vec<SyntaxTree::Value>,
    vars : &mut HashMap<String, (u32, SyntaxTree::ValueType)>,
    var_num : &mut u32, 
    ret_val : &mut u32) -> String {

    let mut ret = String::from("");
    let mut inits = String::from("");

    for x in children {
        match x.v_type {
            SyntaxTree::ValueType::CALL => {
                
            },
            SyntaxTree::ValueType::VARIABLE => {
                ret.push_str(&format!("PUSH {}\n",
                    vars.get(&x.name
                    .clone()
                    .expect("Error: Variable in expression doesn't have a name"))
                    .unwrap().0));

            },
            SyntaxTree::ValueType::ADD => {
                ret.push_str("ADD\n");
            },
            SyntaxTree::ValueType::SUB => {
                ret.push_str("SUB\n");
            },
            SyntaxTree::ValueType::DIV => {
                ret.push_str("DIV\n");
            },
            SyntaxTree::ValueType::MUL => {
                ret.push_str("MUL\n");
            },
            SyntaxTree::ValueType::MOD => {
                ret.push_str("MOD\n");
            },
            SyntaxTree::ValueType::AND => {
                ret.push_str("AND\n");
            },
            SyntaxTree::ValueType::OR => {
                ret.push_str("OR\n");
            },
            SyntaxTree::ValueType::XOR => {
                ret.push_str("XOR\n");
            },
            SyntaxTree::ValueType::EQ => {
                ret.push_str("CMP\n");
            },
            SyntaxTree::ValueType::GREATER => {
                ret.push_str("CMPG\n");
            },
            SyntaxTree::ValueType::GREATEREQ => {
                ret.push_str("CMPG\n");
            },
            SyntaxTree::ValueType::LESS => {
                ret.push_str("CMPL\n");
            },
            SyntaxTree::ValueType::LESSEQ => {
                ret.push_str("CMPL\n");
            },
            _ => {}
        }
    }

    ret

}