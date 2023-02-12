use myminis::expression::evaluator::*;
use myminis::expression::ast::*;


fn main() {
  println!("Hello, world!");
  let add = Expr::BinExpr {
    operator: Oper::Add,
    lhs: Box::new(Expr::Int(1)),
    rhs: Box::new(Expr::Int(2)),
  };

  let add2 = Expr::t_add(Expr::Int(3), Expr::Int(4));

  println!("1 + 2 = {}", evaluate(add));
  println!("3 + 4 = {}", evaluate(add2));
}
