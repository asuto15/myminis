pub static PLUS: &str = "+";

pub enum Expr {
  BinExpr {
    operator: Oper,
    lhs: Box<Expr>,
    rhs: Box<Expr>,
  },
  Int(i8),
}

impl Expr {
  pub fn t_add(a: Expr, b: Expr) -> Expr {
    Expr::BinExpr {
      operator: Oper::Add,
      lhs: Box::new(a),
      rhs: Box::new(b)
    }
  }

  pub fn t_sub(a: Expr, b: Expr) -> Expr {
    Expr::BinExpr {
      operator: Oper::Sub,
      lhs: Box::new(a),
      rhs: Box::new(b)
    }
  }

  pub fn t_mul(a: Expr, b: Expr) -> Expr {
    Expr::BinExpr {
      operator: Oper::Mul,
      lhs: Box::new(a),
      rhs: Box::new(b)
    }
  }

  pub fn t_div(a: Expr, b: Expr) -> Expr {
    Expr::BinExpr {
      operator: Oper::Div,
      lhs: Box::new(a),
      rhs: Box::new(b)
    }
  }

  pub fn t_int(value: i8) -> Expr {
    Expr::Int(value)
  }
}

pub enum Oper {
  Add,
  Sub,
  Mul,
  Div,
}

// pub struct BinExpr {
//   _type: String,
//   pub operator: String,
//   pub lhs: Expr,
//   pub rhs: Expr,
// }

// impl BinExpr {
//   pub fn new(operator: &str, lhs: Expr, rhs: Expr) -> BinExpr {
//     BinExpr {String::from("BinExpr"), operator, lhs, rhs}
//   }

//   pub fn tAdd(a: Expr, b: Expr) -> BinExpr {
//     BinExpr::new(String::from("+"), a, b)
//   }

  // pub fn tSub(a: &str, b: &str) -> BinExpr {
  //   BinExpr::new("-", a, b)
  // }

  // pub fn tMul(a: &str, b: &str) -> BinExpr {
  //   BinExpr::new("*", a, b)
  // }

  // pub fn tDiv(a: &str, b: &str) -> BinExpr {
  //   BinExpr::new('/', a, b)
  // }
// }

// pub struct Int {
//   _type: String,
//   pub value: i8,
// }

// impl Int {
//   fn new(value: i8) -> Int {
//     Int{String::from("IntegerLiteral"), value}
//   }

//   pub fn tInt(value: i8) -> Int {
//     Int::new(value)
//   }
// }