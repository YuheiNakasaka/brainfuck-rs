#[derive(PartialEq, Eq, Clone, Debug)]
pub enum AstType {
    PointerIncrement,
    PointerDecrement,
    ValueIncrement,
    ValueDecrement,
    Loop,
    Output,
    Input,
    None,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Ast {
    pub ast_type: AstType,
    pub children: Vec<Ast>,
}

impl Ast {
    pub fn new(ast_type: AstType, children: Vec<Ast>) -> Ast {
        Ast {
            ast_type: ast_type,
            children: children,
        }
    }
}
