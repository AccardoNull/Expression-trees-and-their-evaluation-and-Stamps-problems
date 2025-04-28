// Starter file provided to CSC 330, Spring 2025, Assignment 3
// Copyright Mike Zastre, UVic 2025.
//
// This echoes the functionality provided by the starter file in
// Haskell for the similar problem in Assignment 1.
//
// Therefore your task is to complete the functionality needed
// for the ParseNode datatype definition (i.e. completing
// the `enum` statement) as well as for operations on the
// datatype (i.e., completing the `eval` function on ParseNode.)
//

#[derive(Debug, PartialEq)]

// The `derive` above will help as the `Debug` trait will help
// produce useful output when trying to print a `ParseNode`.
//
// Similarly the `PartialEq` trait will provide default implementations
// of such opertions at `==` and `!=`.
//

enum ParseNode {
    Number(i64),
    Add(Box<ParseNode>, Box<ParseNode>),
    Sub(Box<ParseNode>, Box<ParseNode>),
    Mul(Box<ParseNode>, Box<ParseNode>),
    Div(Box<ParseNode>, Box<ParseNode>),
    Exp(Box<ParseNode>, Box<ParseNode>),
}

impl ParseNode {
    fn eval(&self) -> i64 {
        match self {
            ParseNode::Number(n) => *n,
            ParseNode::Add(left, right) => left.eval() + right.eval(),
            ParseNode::Sub(left, right) => left.eval() - right.eval(),
            ParseNode::Mul(left, right) => left.eval() * right.eval(),
            ParseNode::Div(left, right) => left.eval() / right.eval(),
            ParseNode::Exp(left, right) => left.eval().pow(right.eval() as u32),
        }
    }
}

fn main() {
    // Example expressions matching Haskell's parse_a, parse_b, parse_c, parse_d
    
    let parse_z = ParseNode::Number(42);
    println!("Result of parse_z: {}", parse_z.eval()); // Should print 42


    // Code for expression (a) in the assignment?
    println!(
        "Result of parse_a: {}",
        ParseNode::Add(
            Box::new(ParseNode::Number(2)),
            Box::new(ParseNode::Mul(
                Box::new(ParseNode::Number(3)),
                Box::new(ParseNode::Number(4)),
            )),
        )
        .eval()
    );

    // Code for expression (b) in the assignment?
    println!(
        "Result of parse_b: {}",
        ParseNode::Mul(
            Box::new(ParseNode::Add(
                Box::new(ParseNode::Number(3)),
                Box::new(ParseNode::Number(5)),
            )),
            Box::new(ParseNode::Sub(
                Box::new(ParseNode::Number(7)),
                Box::new(ParseNode::Number(2)),
            )),
        )
        .eval()
    );

    // Code for expression (c) in the assignment?
    println!(
        "Result of parse_c: {}",
        ParseNode::Sub(
            Box::new(ParseNode::Add(
                Box::new(ParseNode::Number(4)),
                Box::new(ParseNode::Mul(
                    Box::new(ParseNode::Number(6)),
                    Box::new(ParseNode::Exp(
                        Box::new(ParseNode::Number(2)),
                        Box::new(ParseNode::Number(3)),
                    )),
                )),
            )),
            Box::new(ParseNode::Div(
                Box::new(ParseNode::Number(5)),
                Box::new(ParseNode::Add(
                    Box::new(ParseNode::Number(1)),
                    Box::new(ParseNode::Number(1)),
                )),
            )),
        )
        .eval()
    );

    // Code for expression (d) in the assignment?
    println!(
        "Result of parse_d: {}",
        ParseNode::Sub(
            Box::new(ParseNode::Add(
                Box::new(ParseNode::Number(14)),
                Box::new(ParseNode::Mul(
                    Box::new(ParseNode::Number(26)),
                    Box::new(ParseNode::Exp(
                        Box::new(ParseNode::Number(12)),
                        Box::new(ParseNode::Number(3)),
                    )),
                )),
            )),
            Box::new(ParseNode::Div(
                Box::new(ParseNode::Number(55)),
                Box::new(ParseNode::Add(
                    Box::new(ParseNode::Number(10)),
                    Box::new(ParseNode::Number(11)),
                )),
            )),
        )
        .eval()
    );
    
}
