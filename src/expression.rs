pub mod ast;
pub mod evaluator;

#[cfg(test)]
mod tests {
  use crate::expression::ast::*;
  use crate::expression::evaluator::*;
  #[test]
  fn add() {
    let expr = Expr::t_add(Expr::Int(1), Expr::Int(2));
    assert_eq!(3, evaluate(expr));
  }

  #[test]
  fn sub() {
    let expr = Expr::t_sub(Expr::Int(5), Expr::Int(3));
    assert_eq!(2, evaluate(expr));
  }

  #[test]
  fn mul() {
    let expr = Expr::t_mul(Expr::Int(2), Expr::Int(3));
    assert_eq!(6, evaluate(expr));
  }

  #[test]
  fn div() {
    let expr = Expr::t_div(Expr::Int(10), Expr::Int(2));
    assert_eq!(5, evaluate(expr));
  }
}