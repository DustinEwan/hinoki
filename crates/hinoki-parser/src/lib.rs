mod format_rules;

use once_cell::sync::OnceCell;
use tracing_subscriber::{fmt::FormattedFields, util::SubscriberInitExt};

use pest::Parser;
use pest::{error::Error, pratt_parser::PrattParser};
use pest_derive::Parser;

use crate::format_rules::format_parse_error;

#[derive(Parser)]
#[grammar = "./hinoki.pest"]
struct HinokiParser;

// lazy_static::lazy_static! {
//     static ref PRATT_PARSER: PrattParser<Rule> = {
//         use pest::pratt_parser::{Assoc::*, Op};
//         use Rule::*;
//
//         // Precedence is defined lowest to highest
//         PrattParser::new()
//             .op(Op::infix(xor, Left))
//             .op(Op::infix(bitwise_or, Left))
//             .op(Op::infix(bitwise_and, Left))
//             .op(Op::infix(not, Left))
//             .op(Op::infix(less_than, Left) | Op::infix(greater_than, Left) | Op::infix(equal, Left) | Op::infix(less_than_or_equal, Left) | Op::infix(greater_than_or_equal, Left) | Op::infix(not_equal, Left) )
//             .op(Op::infix(shift_left, Left) | Op::infix(shift_right, Left))
//             .op(Op::infix(add, Left) | Op::infix(subtract, Left))
//             .op(Op::infix(modulus, Left))
//             .op(Op::infix(multiply, Left) | Op::infix(divide, Left))
//             .op(Op::infix(power, Left))
//     };
// }

pub struct Program {
    pub instructions: Vec<Instruction>,
}

pub enum Instruction {
    Statement(Statement),
    Expression,
}

pub enum Statement {
    Declaration(DeclarationStmt),
    Assignment(AssignmentStmt),
    Return(ReturnStmt),
    Expression(Expression),
}

pub struct DeclarationStmt {}
pub struct AssignmentStmt {}
pub struct ReturnStmt {}

pub enum Expression {
    Integer(i64),
    Float(f64),
    Binary(BinaryExpr),
}

pub struct BinaryExpr {
    lhs: Box<Expression>,
    op: Op,
    rhs: Box<Expression>,
}

pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
    Mod,
    Power,

    ShiftLeft,
    ShiftRight,
    BitwiseAnd,
    BitwiseOr,
    Xor,
    Not,

    LessThan,
    GreaterThan,
    Equal,
    LessThanOrEqual,
    GreaterThanOrEqual,
    NotEqual,
}

// pub fn parse_expr(pairs: Pairs<Rule>) -> Expression {
//     PRATT_PARSER
//         .map_primary(|primary| match primary.as_rule() {
//             Rule::
//         })
// }

fn snapshot_parsing(input: &str) -> String {
    let file = HinokiParser::parse(Rule::file, input);

    match file {
        Ok(mut file) => format!("{:#?}", file.next().unwrap()),
        Err(e) => {
            let formatted_err = format_parse_error(e.clone());
            println!("{}", e);
            format!("{:#?}\n\n{}", e, formatted_err)
        }
    }
}

pub fn setup_trace() {
    static INSTANCE: OnceCell<()> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .without_time()
            .with_target(false)
            .finish()
            .init();
    });
}

#[cfg(test)]
mod test {
    use snap_builder::build_snaps;

    use crate::*;

    macro_rules! snap {
        ($name: tt) => {
            #[test]
            fn $name() {
                setup_trace();

                let contents =
                    include_str!(concat!("../testdata/snapshots/", stringify!($name), ".hi"));
                let mut settings = insta::Settings::clone_current();

                settings.set_snapshot_path("../testdata/output/");
                settings.bind(|| {
                    insta::assert_snapshot!(snapshot_parsing(contents));
                });
            }
        };
    }

    build_snaps!();
    // snap!(test_declaration, "simple_arithmetic.hi");
}
