#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Operator {
    Sum,
    Sub,
    Mul,
    Div,
    Exp,
    Sci,
    Neg,
    Pos,
}

impl Operator {
    pub fn have_precedence(&self, op: &Operator) -> std::cmp::Ordering {
        precedence(self).cmp(&precedence(op))
    }

    pub fn is_left_associative(&self) -> bool {
        match self {
            Operator::Neg => false,
            Operator::Pos => false,
            Operator::Exp => false,
            Operator::Mul => true,
            Operator::Div => true,
            Operator::Sci => false,
            Operator::Sum => true,
            Operator::Sub => true,
        }
    }
}

fn precedence(op: &Operator) -> u8 {
    // PEMDAS precedence of operators:
    //
    // 1) NEG POS   <- highest
    // 2) ^
    // 3) * / E
    // 4) - +       <- lowest
    match op {
        Operator::Neg => 3,
        Operator::Pos => 3,
        Operator::Exp => 2,
        Operator::Mul => 1,
        Operator::Div => 1,
        Operator::Sci => 1,
        Operator::Sum => 0,
        Operator::Sub => 0,
    }
}

#[derive(Debug)]
pub enum Data {
    Op(Operator),
    Val(f64),
}