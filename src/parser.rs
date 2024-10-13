use std::borrow::Borrow;

use aho_corasick::AhoCorasick;
/// # Parser
///
/// The idea of this language is to have "chaining" syntax like the following
///
/// ```text
/// 1 + 2 + 3 as x;
/// ```
///
/// When writing an expression, appending additional transformations should be
/// done by ONLY appending additional syntax to the end of the expression.
///
/// ```text
/// [1 2 3] map x { x + 1 } map x { x * 2 } map x { x - 1 } as y;
/// ```
///
/// This is a simple example of chaining syntax. The idea is that the expression
/// is evaluated from left to right, and the result of the expression is passed
/// to the next transformation.
///
/// In keeping with this idea, we do away with BODMAS
///
/// ```text
/// 1 + 2 * 3 == 9 as is_true;
/// ```
///
/// Semi-colons are used to denote the end of an expression.
///
/// ## In-fix operators
/// While most things are applied post-fix, most mathematical operators take two
/// operands. Making a pure "chaining" pattern for that is not really possible.
///
/// Instead, in-fix operators are a special case where the right hand side must
/// be a simple expression.
///
/// ```
/// # literals are simple expressions
/// 1 + 2 + 3
/// # variable names are simple expressions
/// a + b + c
/// ```
///
/// Functions with multiple args can use parens to take the extra args:
///
/// ```
/// a scale(2) repeat{ count: 3 }
/// ```
use chumsky::{extra::Full, prelude::*, primitive};

#[derive(Debug, Clone)]
enum BracketedNode<'inp> {
    Str(&'inp str),
    Round(Vec<BracketedNode<'inp>>),
    Square(Vec<BracketedNode<'inp>>),
    Curly(Vec<BracketedNode<'inp>>),
    Angle(Vec<BracketedNode<'inp>>),
    InvalidRound(&'inp str),
    InvalidSquare(&'inp str),
    InvalidCurly(&'inp str),
    InvalidAngle(&'inp str),
}

#[derive(Debug, Clone)]
struct BlockInner<'inp> {
    statements: Vec<Expr<'inp>>,
    expr: Option<Box<Expr<'inp>>>,
}

#[derive(Debug, Clone)]
enum Token {
    PLUS,
}

#[derive(Debug, Clone)]
enum Literal {
    Num(f64),
    Str(&'static str),
    Bool(bool),
}

#[derive(Debug, Clone)]
enum InfixOp {
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /
    Exp, // **
}

#[derive(Debug, Clone)]
enum Expr<'inp> {
    InfixOp(InfixOp, Box<Expr<'inp>>),
    Literal(Literal),
    Paren(Box<Expr<'inp>>),
    Var(&'inp str),
    Assign(Box<Location<'inp>>, Box<Expr<'inp>>),
    Block(BlockInner<'inp>),
}

#[derive(Debug, Clone)]
enum Location<'inp> {
    Var(&'inp str),
}

#[derive(Debug, Clone)]
struct ParserState {}

#[derive(Debug, Clone)]
struct ParserContext {}

const VAR: f64 = 1.156;

/// # Apply string processing rules.
/// This takes the raw source code of the string and turns it into the string
/// value in the program.
///
/// ## Rules
/// - Replace escapes (\t etc.)
/// - If the first char is a newline, strip it.
/// - Replace leading whitespace up to the indent of the first standalone line.
/// -
///
///
// fn process_str<'inp>(s: &'inp str) -> String {
//     let a = AhoCorasick::builder()
//         .ascii_case_insensitive(false)
//         .build(["\\t", "\\n", "\\r", "\\\"", "\\\\"])
//         .unwrap();
//     a.replace_all(s, &["\t", "\n", "\r", "\"", "\\"])
// }

fn parser<'inp>(
) -> impl Parser<'inp, &'inp str, Expr<'inp>, Full<Rich<'inp, char>, ParserState, ParserContext>> + Clone
{
    let p_number = text::digits(10)
        .to_slice()
        .map(|s: &str| Expr::Literal(Literal::Num(s.parse().unwrap())));

    let p_bool = choice((
        just("true").map(|_| Expr::Literal(Literal::Bool(true))),
        just("false").map(|_| Expr::Literal(Literal::Bool(false))),
    ));

    let mut p_expr = Recursive::declare();
    let mut p_chain = Recursive::declare();
    let mut p_simpleexpr = Recursive::declare();
    let mut p_infix = Recursive::declare();

    let parse_infix = |s: &'static str, op: InfixOp| {
        just(s)
            .ignore_then(p_simpleexpr)
            .map(|(state, e)| Expr::InfixOp(op, Box::new(e)))
    };

    p_infix.define(choice((
        parse_infix("+ ", InfixOp::Add),
        parse_infix("- ", InfixOp::Add),
        parse_infix("* ", InfixOp::Add),
        parse_infix("/ ", InfixOp::Add),
    )));

    p_simpleexpr.define(just("(").ignore_then(p_expr).then_ignore(just(")")));

    // TODO: Strings...
    // escapes
    // multiline & whitespace behaviour
    // f-strings

    p_chain.define(
        p_expr
            .repeated()
            .separated_by(just(" "))
            .collect::<Vec<_>>(),
    );

    p_chain
}

// fn parse_nonbracketed<'inp>() -> impl Parser<
//     'inp,
//     &'inp str,
//     BracketedNode<'inp>,
//     Full<Rich<'inp, char>, ParserState, ParserContext>,
// > + Clone {
//     primitive::none_of("()[]{}")
//         .repeated()
//         .to_slice()
//         .map(BracketedNode::Str)
// }

// fn pb<'inp>() -> impl Parser<
//     'inp,
//     &'inp str,
//     Vec<BracketedNode<'inp>>,
//     Full<Rich<'inp, char>, ParserState, ParserContext>,
// > {
//     recursive(|pb_| {
//         choice((
//             parse_nonbracketed(),
//             pb_.clone()
//                 .delimited_by(just("{"), just("}"))
//                 .map(BracketedNode::Curly),
//             pb_.clone()
//                 .delimited_by(just("["), just("]"))
//                 .map(BracketedNode::Square),
//             pb_.clone()
//                 .delimited_by(just("("), just(")"))
//                 .map(BracketedNode::Curly),
//         ))
//         .repeated()
//         .collect()
//     })
// }
