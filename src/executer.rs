use std::io::{stdin, stdout, Write};

use crate::ast::{Ast, AstType};

pub fn execute(asts: Vec<Ast>) {
    let mut memory: Vec<u8> = [0; 1024].to_vec();
    let mut code_ptr: usize = 0;
    inner_execute(&asts, &mut memory, &mut code_ptr);
}

fn inner_execute(asts: &Vec<Ast>, memory: &mut Vec<u8>, code_ptr: &mut usize) {
    for ast in asts {
        match ast.ast_type {
            AstType::PointerIncrement => {
                *code_ptr += 1;
            }
            AstType::PointerDecrement => {
                *code_ptr -= 1;
            }
            AstType::ValueIncrement => {
                memory[*code_ptr] += 1;
            }
            AstType::ValueDecrement => {
                memory[*code_ptr] -= 1;
            }
            AstType::Output => {
                print!("{}", memory[*code_ptr] as char);
                stdout().flush().unwrap();
            }
            AstType::Loop => {
                while memory[*code_ptr] != 0 {
                    inner_execute(&ast.children, memory, code_ptr);
                }
            }
            AstType::Input => {
                let mut s = String::new();
                stdin().read_line(&mut s).unwrap();
                let s = s.trim_end();
                if s.len() > 0 {
                    memory[*code_ptr] = s.as_bytes()[0];
                } else {
                    memory[*code_ptr] = 10u8;
                }
            }
            _ => {}
        }
    }
}
