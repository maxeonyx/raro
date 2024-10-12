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

use chumsky::{extra::Full, prelude::*};

#[derive(Debug, Clone)]
struct BlockInner {
    statements: Vec<Expr>,
    expr: Option<Box<Expr>>,
}


#[derive(Debug, Clone)]
enum Literal {
    Num(f64),
    Str(String),
    Bool(bool),
}

#[derive(Debug, Clone)]
enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Literal(Literal),
    Var(String),
    Assign(Box<Expr>, String),
    Block(BlockInner),
}

struct ParserState {}

struct ParserContext {}

fn parser<'inp>(input: &'inp str) -> impl Parser<'inp, &'inp str, BlockInner, Full<Rich<'inp, char>, ParserState, ParserContext>> {

    just("a").to(BlockInner {
        statements: vec![],
        expr: None,
    })

}
