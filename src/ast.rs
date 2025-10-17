#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i32),
    Identifier(String),
    Binary {
        left: Box<Expr>,
        operator: String,
        right: Box<Expr>,
    },
    Call {
        name: String,
        args: Vec<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum Stmt {
    /// let name: type = value;
    VarDecl {
        name: String,
        var_type: String,
        value: Expr,
    },

    /// name = value;
    Assignment {
        name: String,
        value: Expr,
    },

    /// if condition { then_branch } else { else_branch_opt }
    IfStmt {
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
    },

    /// while condition { body }
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },

    /// function definition: fn name(params) -> ret_type { body }
    Function {
        name: String,
        params: Vec<(String, String)>, // (param_name, param_type)
        ret_type: String,
        body: Vec<Stmt>,
    },

    /// return expr_opt;
    Return(Option<Expr>),

    /// expression statement (e.g., a call on its own)
    ExprStmt(Expr),
}
