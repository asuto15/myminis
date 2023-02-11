use super::ast::*;

pub fn evaluate(expr: Expr) -> i8 {
  match expr {
    Expr::BinExpr { operator, lhs, rhs } => {
      match operator {
        Oper::Add => evaluate(*lhs) + evaluate(*rhs),
        Oper::Sub => evaluate(*lhs) - evaluate(*rhs),
        Oper::Mul => evaluate(*lhs) * evaluate(*rhs),
        Oper::Div => evaluate(*lhs) / evaluate(*rhs),
      }
    }
    Expr::Int(value) => value,
  }
}