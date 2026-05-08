use std::io;
use calculator::lexer::Tokens;
use calculator::parser::{Data, Operator, RPN};

fn main() {
    let mut expression = String::new();

    io::stdin()
        .read_line(&mut expression)
        .expect("Failed to read line.");

    println!("Input an expression: {}", expression.trim());

    let expression = Tokens::from(expression.trim()).expect("Could not tokeinze the expression.");

    println!("\nTokens: {:?}", expression.tokens);

    let rpn = RPN::from(expression).expect("Some error occured.");
    println!("\nrpn: {:?}", rpn.data);

    let mut stack: Vec<f64> = Vec::new();

    for data in rpn.data {
        if let Data::Op(op) = data {
            let v1 = stack.pop().unwrap();
            let v2 = stack.pop().unwrap();
            let result = match op {
                Operator::Sum => v1 + v2,
                Operator::Sub => v2 - v1,
                Operator::Mul => v1 * v2,
                Operator::Div => v2 / v1,
                Operator::Sci => v2 * 10_f64.powf(v1),
                Operator::Exp => v2.powf(v1),
                Operator::Neg => -v1,
                Operator::Pos => +v1,
            };
            stack.push(result);
        } else if let Data::Val(v) = data {
            stack.push(v);
        }
    }

    println!("\nExpression result: {}", stack.pop().unwrap());
}
