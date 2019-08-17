use super::super::tokenizer::tokenizer as Token;
use std::collections::HashMap;
use std::cell::RefCell;

fn advance(tokens : &Vec<Token::Token>, index : &mut usize) -> Option<Token::Token> {
    *index+=1;
    match tokens.get(*index) {
        Some(v) => {Some(v.clone())},
        None => {None}
    }
}

fn peek (tokens : &Vec<Token::Token>, index : usize) -> Option<Token::Token> {
    let index = index+1;
    match tokens.get(index) {
        Some(v) => {Some(v.clone())},
        None => {None}
    }
}

fn match_expr(tokens : &Vec<Token::Token>, index : usize) -> Vec<Token::Token> {

    let mut i = index;
    let mut new_vec : Vec<Token::Token> = Vec::new();
    loop {
        new_vec.push(tokens[i].clone());
        i+=1;
        
        match tokens[i].t_type { Token::TokenType::SEMICOLON => {break}, _ => {}}
    }

    new_vec
}

fn match_expr_open(tokens : &Vec<Token::Token>, index : usize) -> Vec<Token::Token> {

    let mut i = index;
    let mut new_vec : Vec<Token::Token> = Vec::new();
    loop {
        new_vec.push(tokens[i].clone());
        i+=1;
        
        match tokens[i].t_type { Token::TokenType::OPENBLOCK => {break}, _ => {}}
    }

    new_vec
}

fn parse_expr(tokens : Vec<Token::Token>) -> Value {

    let mut base = Value {
        v_type : ValueType::EXPRESSION,
        name : None,
        children : Vec::new()
    };

    let mut x = 0;
    let mut expr : Vec<Value> = Vec::new();
    let mut op_stack : Vec<Value> = Vec::new();

    while x < tokens.len() {

        match tokens[x].t_type {
            Token::TokenType::OPENBRACKET => {
                op_stack.push(Value {
                    v_type : ValueType::OPEN,
                    name : None,
                    children : Vec::new()
                })
            },
            Token::TokenType::CLOSEBRACKET => {
                let mut curr = op_stack.pop().expect("Error: Mismatched brackets in expression");
                while match curr.v_type {
                    ValueType::OPEN => {
                        false
                    },
                    _ => {
                        expr.push(curr);
                        curr = op_stack.pop().expect("Error: Mismatched brackets in expression");
                        true
                    }
                } {}
            },
            Token::TokenType::ADD | Token::TokenType::SUB | 
            Token::TokenType::NOT | Token::TokenType::MUL | 
            Token::TokenType::DIV | Token::TokenType::AND | 
            Token::TokenType::OR  | Token::TokenType::XOR => {
                while tokens[x].t_type.get_precendence() < op_stack.last()
                    .unwrap_or(&Value {v_type:ValueType::OPEN, name : None, children : Vec::new()})
                    .get_precendence() {
                    
                    expr.push(op_stack.pop().expect("Error: lmao idk"));
                }
                op_stack.push(Value {v_type : ValueType::from_token(tokens[x].clone().t_type), name : None, children : Vec::new()});
            },
            Token::TokenType::IDENTIFIER => {
                match peek(&tokens, x){
                    Some(v) => {
                        match v.t_type {
                            Token::TokenType::OPENBRACKET => {
                                let f_name = tokens[x].name.clone().expect("Error: Function call doesn't have a name");
                                let mut f_value = Value {
                                    v_type : ValueType::CALL,
                                    name : Some(f_name),
                                    children : Vec::new()
                                };
                                x+=1;
                                enum FunctionState {
                                    IDEN,
                                    COMMA,
                                    OPEN
                                }
                                let mut f_state = FunctionState::OPEN;
                                while match advance(&tokens, &mut x) {
                                    Some(v) => {
                                        match v.t_type {
                                        Token::TokenType::IDENTIFIER => {
                                            f_value.children.push(Value {
                                                v_type : ValueType::VARIABLE,
                                                name : v.clone().name,
                                                children : Vec::new()
                                            });
                                            match f_state {
                                                FunctionState::IDEN => {panic!("Error: Unexpected token in function call")},
                                                _ => {
                                                    f_state = FunctionState::IDEN;
                                                    true
                                                }
                                            }
                                        },
                                        Token::TokenType::COMMA => {
                                            match f_state {
                                                FunctionState::OPEN | FunctionState::COMMA => {panic!("Error: Unexpected token in function call")},
                                                _ => {
                                                    f_state = FunctionState::COMMA;
                                                    true
                                                }
                                            }
                                        },
                                        Token::TokenType::CLOSEBRACKET => {
                                            match f_state {
                                                FunctionState::COMMA => {panic!("Error: Unexpected token in function call")},
                                                _ => {false}
                                            }
                                        },
                                        _ => {panic!("Error: Unexpected token in function call")}
                                    }},
                                    None => {false}
                                } {}
                                expr.push(f_value);
                            },
                        _ => {expr.push(Value {
                                v_type : ValueType::VARIABLE,
                                name : tokens[x].name.clone(),
                                children : Vec::new()
                            })}
                        }
                    },
                    None => {}
                }
            },
            _ => {panic!("Error: Invalid expression")}
        }
    }

    for x in op_stack {
        if x.get_precendence() != -1 {
            expr.push(x);
        }
    }

    base.children = expr;
    base
}

pub enum ValueType {
    STATEMENT,
    EXPRESSION,
    VALUE,
    OPERATOR,
    RETURN,
    FUNCTION,
    ARGUMENT,
    PARAMETER,
    DECLARATION,
    ASSIGNMENT,
    VARIABLE,
    CONDITION,
    IF,
    ELSE,
    WHILE,
    ADDRESS,
    GOTO,
    NUM,
    STR,
    LIST,
    CALL,
    ADD,
    SUB,
    MUL,
    MOD,
    DIV,
    NOT,
    AND,
    OR,
    XOR,
    OPEN,
    CLOSE
}

impl ValueType {
    pub fn get_precendence(&self) -> i32{
        match self {
            ValueType::OR | ValueType::XOR => {1},
            ValueType::AND => {2},
            ValueType::ADD | ValueType::SUB => {3},
            ValueType::MUL | ValueType::DIV | ValueType::MOD => {4},
            ValueType::NOT => {5},
            _ => {-1}
        }
    }

    pub fn from_token(token : Token::TokenType) -> ValueType {
        match token {
            Token::TokenType::ADD => {ValueType::ADD},
            Token::TokenType::SUB => {ValueType::SUB},
            Token::TokenType::MUL => {ValueType::MUL},
            Token::TokenType::DIV => {ValueType::DIV},
            Token::TokenType::MOD => {ValueType::MOD},
            Token::TokenType::NOT => {ValueType::NOT},
            Token::TokenType::AND => {ValueType::AND},
            Token::TokenType::OR => {ValueType::OR},
            Token::TokenType::XOR => {ValueType::XOR},
            _ => {panic!("Error: Conversion not implemented")}
        }
    }
}

impl Clone for ValueType {
    fn clone(&self) -> ValueType {

        match self {
            ValueType::IF => {ValueType::IF},
            ValueType::ELSE => {ValueType::ELSE},
            ValueType::WHILE => {ValueType::WHILE},
            ValueType::NUM => {ValueType::NUM},
            ValueType::STR => {ValueType::STR},
            ValueType::STATEMENT => {ValueType::STATEMENT},
            ValueType::EXPRESSION => {ValueType::EXPRESSION},
            ValueType::OPERATOR => {ValueType::OPERATOR},
            ValueType::ADDRESS => {ValueType::ADDRESS},
            ValueType::GOTO => {ValueType::GOTO},
            ValueType::CONDITION => {ValueType::CONDITION},
            ValueType::VARIABLE => {ValueType::VARIABLE},
            ValueType::ASSIGNMENT => {ValueType::ASSIGNMENT},
            ValueType::ARGUMENT => {ValueType::ARGUMENT},
            ValueType::FUNCTION => {ValueType::FUNCTION},
            ValueType::VALUE => {ValueType::VALUE},
            ValueType::RETURN => {ValueType::RETURN},
            ValueType::PARAMETER => {ValueType::PARAMETER},
            ValueType::DECLARATION => {ValueType::DECLARATION},
            ValueType::ADD => {ValueType::ADD},
            ValueType::SUB => {ValueType::SUB},
            ValueType::MUL => {ValueType::MUL},
            ValueType::MOD => {ValueType::MOD},
            ValueType::DIV => {ValueType::DIV},
            ValueType::NOT => {ValueType::NOT},
            ValueType::AND => {ValueType::AND},
            ValueType::OR => {ValueType::OR},
            ValueType::XOR => {ValueType::XOR},
            ValueType::CALL => {ValueType::CALL},
            ValueType::OPEN => {ValueType::OPEN},
            ValueType::CLOSE => {ValueType::CLOSE},
            ValueType::LIST => {ValueType::LIST}
        }
    }
}

pub struct Value {
    v_type : ValueType,
    name : Option<String>,
    children : Vec<Value>
}

impl Clone for Value {
    fn clone(&self) -> Value {

        Value {
            name : self.name.clone(),
            v_type : self.v_type.clone(),
            children : self.children.clone()
        }
    }
}

impl Value {
    pub fn new_function() -> Value {
        Value {
            v_type : ValueType::FUNCTION,
            name : None,
            children : Vec::new()
        }
    }

    pub fn get_left(&self) -> Value {
        (*self.children.get(0).expect("Error: No left node")).clone()
    }

    pub fn get_precendence(&self) -> i32 {
        self.v_type.get_precendence()
    }
}

enum Statement {
    FUNCTION(String),
    IF(String),
    WHILE(String)
}

pub struct AST {
    functions : Vec<Value>
}

impl AST {
    pub fn new() -> AST {
        AST {
            functions : Vec::new()
        }
    }

    pub fn generate_tree(&mut self, tokens: Vec<Token::Token>) {

        let mut index = 0;
        let mut ast = AST::new();
        let mut curr_func = Value::new_function();
        let mut curr_name = String::from("");
        let mut blocks : HashMap<String, RefCell<Value>> = HashMap::new();
        let mut block_stack : Vec<Statement> = Vec::new();
        let mut conditional_num : u64 = 0;

        while index < tokens.len() {
            match tokens[index].t_type {
                Token::TokenType::FUNCTION => {

                    curr_func = Value::new_function();
                    let next = advance(&tokens, &mut index).expect("Error: Invalid token");
                    match next.t_type {Token::TokenType::IDENTIFIER => {}, _ => {panic!("Error: Invalid token")}}
                    curr_func.name = next.name;
                    if !advance(&tokens, &mut index).expect("Error: Invalid token").t_type.is_openblock() {panic!("Error: Invalid token")} 
                    block_stack.push(Statement::FUNCTION(curr_func.name.expect("Error: Can't parse function name")));
                    let name = tokens[index].clone().name.expect("Error: Can't parse function name");
                    blocks.insert(name.clone(), RefCell::from(Value {
                        v_type : ValueType::FUNCTION,
                        name : Some(name.clone()),
                        children : Vec::new()
                    }));
                    curr_name = name.clone();
                },
                Token::TokenType::NUM => {

                    let name = advance(&tokens, &mut index).expect("Error: Invalid token").name;

                    let declaration = Value {
                        v_type : ValueType::NUM,
                        name : name.clone(),
                        children : Vec::new()
                    };

                    blocks.get(curr_name.as_str()).expect("Error: Declaration out of block").borrow_mut().children.push(declaration);

                    match advance(&tokens, &mut index).expect("Error: Invalid token").t_type {
                        Token::TokenType::ASSIGN => {
                            let expr = match_expr(&tokens, index);
                            let assignment = Value{v_type : ValueType::ASSIGNMENT, name : name.clone(), children : vec!{parse_expr(expr)}};
                            blocks.get(curr_name.as_str()).expect("Error: Assignment out of block").borrow_mut().children.push(assignment);
                        },
                        Token::TokenType::SEMICOLON => {},
                        _ => {panic!("Error: Invalid token")}
                    }

                },
                Token::TokenType::STR => {

                    let name = advance(&tokens, &mut index).expect("Error: Invalid token").name;

                    let declaration = Value {
                        v_type : ValueType::STR,
                        name : name.clone(),
                        children : Vec::new()
                    };

                    blocks.get(curr_name.as_str()).expect("Error: Declaration out of block").borrow_mut().children.push(declaration);

                    match advance(&tokens, &mut index).expect("Error: Invalid token").t_type {
                        Token::TokenType::ASSIGN => {
                            let expr = match_expr(&tokens, index);
                            let assignment = Value{v_type : ValueType::ASSIGNMENT, name : name.clone(), children : vec!{parse_expr(expr)}};
                            blocks.get(curr_name.as_str()).expect("Error: Assignment out of block").borrow_mut().children.push(assignment);
                        },
                        Token::TokenType::SEMICOLON => {},
                        _ => {panic!("Error: Invalid token")}
                    }
                },
                Token::TokenType::LIST => {
                    let name = advance(&tokens, &mut index).expect("Error: Invalid token").name;

                    let declaration = Value {
                        v_type : ValueType::LIST,
                        name : name.clone(),
                        children : Vec::new()
                    };

                    blocks.get(curr_name.as_str()).expect("Error: Declaration out of block").borrow_mut().children.push(declaration);

                    match advance(&tokens, &mut index).expect("Error: Invalid token").t_type {
                        Token::TokenType::SEMICOLON => {},
                        _ => {panic!("Error: Invalid token")}
                    }
                },
                Token::TokenType::WHILE => {

                    let mut while_loop = Value {v_type : ValueType::WHILE, name : None, children : Vec::new()};
                    let expr = match_expr_open(&tokens, index);
                    let conditional = Value{v_type : ValueType::CONDITION, name : Some(String::from("condition")), children : vec!{parse_expr(expr)}};
                    while_loop.children.push(conditional);
                    
                    block_stack.push(Statement::WHILE(format!("while{}", conditional_num)));
                    let name = format!("while{}", conditional_num);
                    blocks.insert(name.clone(), RefCell::from(while_loop));
                    curr_name = name.clone();
                    conditional_num+=1;
                },
                Token::TokenType::IF => {
                    
                    let mut while_loop = Value {v_type : ValueType::IF, name : None, children : Vec::new()};
                    let expr = match_expr_open(&tokens, index);
                    let conditional = Value{v_type : ValueType::CONDITION, name : Some(String::from("condition")), children : vec!{parse_expr(expr)}};
                    while_loop.children.push(conditional);
                    
                    block_stack.push(Statement::IF(format!("if{}", conditional_num)));
                    let name = format!("if{}", conditional_num);
                    blocks.insert(name.clone(), RefCell::from(while_loop));
                    curr_name = name.clone();
                    conditional_num+=1;
                },
                Token::TokenType::IDENTIFIER => {

                    let name = tokens[index].clone().name;

                    match advance(&tokens, &mut index).expect("Error: Invalid token").t_type {
                        Token::TokenType::ASSIGN => {
                            let expr = match_expr(&tokens, index);
                            let assignment = Value{v_type : ValueType::ASSIGNMENT, name : name.clone(), children : vec!{parse_expr(expr)}};
                            blocks.get(curr_name.as_str()).expect("Error: Assignment out of block").borrow_mut().children.push(assignment);
                        },
                        _ => {panic!("Error: Invalid token")}
                    }
                },
                Token::TokenType::GOTO => {

                    let addr = advance(&tokens, &mut index).expect("Error: Invalid token");
                    match addr.t_type{
                        Token::TokenType::ADDR => {
                            let goto = Value{v_type : ValueType::GOTO, name : addr.name, children : Vec::new()};
                            blocks.get(curr_name.as_str()).expect("Error: Goto out of block").borrow_mut().children.push(goto);
                        },
                        _ => {panic!("Error: Invalid token")}
                    }
                },
                Token::TokenType::ADDR => {
                    let name = tokens[index].clone().name;

                    let address = Value{v_type : ValueType::ADDRESS, name : name, children : Vec::new()};
                    blocks.get(curr_name.as_str()).expect("Error: Address out of block").borrow_mut().children.push(address);
                },
                Token::TokenType::CLOSEBLOCK => {
                    match block_stack.pop().expect("Error: Invalid }") {
                        Statement::FUNCTION(val) => {
                            let tmp = val.clone();
                            ast.functions.push((*blocks.get(tmp.as_str()).expect("Error: function not defined")).borrow().clone());
                        },
                        Statement::WHILE(val) => {
                            let tmp = val.clone();
                            match block_stack.last().expect("Error: While loop not in function") {
                                Statement::FUNCTION(var) | Statement::IF(var) | Statement::WHILE(var) => {
                                    blocks.get(var.as_str())
                                    .expect("Error: Funtion is not defined")
                                    .borrow_mut()
                                    .children.push(
                                        (blocks.get(tmp.as_str())
                                        .expect("Error: function not defined"))
                                        .borrow()
                                        .clone());
                                    } 
                            }
                        },
                        Statement::IF(val) => {
                            let tmp = val.clone();
                            match block_stack.last().expect("Error: While loop not in function") {
                                Statement::FUNCTION(var) | Statement::IF(var) | Statement::WHILE(var) => {
                                    blocks.get(var.as_str())
                                    .expect("Error: Funtion is not defined")
                                    .borrow_mut()
                                    .children.push(
                                        (blocks.get(tmp.as_str())
                                        .expect("Error: function not defined"))
                                        .borrow()
                                        .clone());
                                    } 
                            }
                        }
                    }
                }
                _ => {panic!("Error: Invalid token")}
            }
        }
    }
}