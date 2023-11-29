use crate::interpret_bool::*;
use crate::interpret_num::*;
use crate::interpret_cond::*;

pub type N = i32;
pub type B = bool;

#[derive(PartialEq, Debug)]
pub enum Result {
    Num(N),
    Bool(B),
}

#[derive(Debug)]
pub enum Expr {
    BoolExpr(Bool),
    NumExpr(Num),
    CondExpr(Cond)
}

pub fn interpret(expr: Expr) -> Result {
    match expr {
        Expr::NumExpr(x) => Result::Num(interpret_num_expr(x)),
        Expr::BoolExpr(x) => Result::Bool(interpret_bool_expr(x)),
        Expr::CondExpr(x) => interpret_cond_expr(x),
    }
}
