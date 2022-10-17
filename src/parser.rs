use crate::ast::{Ast, AstType};

pub fn parse(src: &str) -> (Vec<Ast>, usize) {
    let mut ast: Vec<Ast> = Vec::new();
    let mut offset = 0;
    let len = src.len();
    let chars = src.chars().collect::<Vec<char>>();
    while offset < len {
        let c = chars[offset];
        match c {
            '>' => ast.push(Ast::new(AstType::PointerIncrement, Vec::new())),
            '<' => ast.push(Ast::new(AstType::PointerDecrement, Vec::new())),
            '+' => ast.push(Ast::new(AstType::ValueIncrement, Vec::new())),
            '-' => ast.push(Ast::new(AstType::ValueDecrement, Vec::new())),
            '.' => ast.push(Ast::new(AstType::Output, Vec::new())),
            '[' => {
                let (result, child_offset) = parse(&src[offset + 1..]);
                ast.push(Ast::new(AstType::Loop, result));
                offset += child_offset;
            }
            ']' => {
                return (ast, offset + 1);
            }
            ',' => ast.push(Ast::new(AstType::Input, Vec::new())),
            _ => {}
        }
        offset += 1;
    }
    return (ast, offset);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let src = "+-><.[++]-,.";
        let ast = parse(src).0;
        assert_eq!(ast.get(0).unwrap().ast_type, AstType::ValueIncrement);
        assert_eq!(ast.get(1).unwrap().ast_type, AstType::ValueDecrement);
        assert_eq!(ast.get(2).unwrap().ast_type, AstType::PointerIncrement);
        assert_eq!(ast.get(3).unwrap().ast_type, AstType::PointerDecrement);
        assert_eq!(ast.get(4).unwrap().ast_type, AstType::Output);
        assert_eq!(ast.get(5).unwrap().ast_type, AstType::Loop);
        assert_eq!(
            ast.get(5).unwrap().children.get(0).unwrap().ast_type,
            AstType::ValueIncrement
        );
        assert_eq!(
            ast.get(5).unwrap().children.get(1).unwrap().ast_type,
            AstType::ValueIncrement
        );
        assert_eq!(ast.get(6).unwrap().ast_type, AstType::ValueDecrement);
        assert_eq!(ast.get(7).unwrap().ast_type, AstType::Input);
        assert_eq!(ast.get(8).unwrap().ast_type, AstType::Output);
    }
}
