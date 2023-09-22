mod format_rules;

use hinoki_ast::{
    BlockExpr, Command, Exclusivity, FunctionDefinition, FunctionParameter, Locality, Mutability,
    Program, TopLevelInstruction, Type, UserDefinedType, Visibility,
};
use once_cell::sync::OnceCell;
use tracing_subscriber::{fmt::FormattedFields, util::SubscriberInitExt};

use pest::{error::Error, iterators::Pair, Parser};
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

// pub fn parse_expr(pairs: Pairs<Rule>) -> Expression {
//     PRATT_PARSER
//         .map_primary(|primary| match primary.as_rule() {
//             Rule::
//         })
// }

pub fn parse(input: &str) -> Result<Program, String> {
    let mut file = match HinokiParser::parse(Rule::file, input) {
        Ok(f) => f,
        Err(e) => {
            return Err(format!("{:#?}", e));
        }
    };

    let mut instructions: Vec<TopLevelInstruction> = vec![];
    for pair in file.next().unwrap().into_inner() {
        let instruction = parse_top_level_instruction(pair)?;
        instructions.push(instruction);
    }

    Ok(Program { instructions })
}

pub fn parse_top_level_instruction(
    pair: pest::iterators::Pair<Rule>,
) -> Result<TopLevelInstruction, String> {
    match pair.as_rule() {
        Rule::function_definition => {
            let function_definition = parse_function_definition(pair.into_inner())?;
            Ok(TopLevelInstruction::FunctionDefinition(function_definition))
        }
        Rule::struct_definition => {
            todo!()
        }
        Rule::enum_definition => {
            todo!()
        }
        Rule::import_stmt => {
            todo!()
        }
        Rule::impl_definition => {
            todo!()
        }
        Rule::trait_definition => {
            todo!()
        }

        Rule::EOI => Ok(TopLevelInstruction::EOI),
        _ => Err(format!("Unexpected top level instruction!\n{:#?}", pair)),
    }
}

macro_rules! step_pair {
    ($pairs: tt, $pair: tt) => {
        $pair = $pairs.next().unwrap();
    };
}

macro_rules! parse_visibility {
    ($pairs: tt, $pair: tt) => {
        match $pair.as_rule() {
            Rule::r#pub => {
                step_pair!($pairs, $pair);
                Visibility::Public
            }
            _ => Visibility::Private,
        }
    };
}

macro_rules! parse_locality {
    ($pairs: tt, $pair: tt) => {
        match $pair.as_rule() {
            Rule::locality => {
                let locality = match $pair.as_str() {
                    "local" => Locality::Local,
                    _ => Locality::Global,
                };

                step_pair!($pairs, $pair);
                locality
            }
            _ => Locality::Global,
        }
    };
}

macro_rules! parse_exclusivity {
    ($pairs: tt, $pair: tt) => {
        match $pair.as_rule() {
            Rule::exclusivity => {
                let exclusivity = match $pair.as_str() {
                    "exclusive" => Exclusivity::Exclusive,
                    "unique" => Exclusivity::Unique,
                    _ => Exclusivity::Shared,
                };

                step_pair!($pairs, $pair);
                exclusivity
            }
            _ => Exclusivity::Shared,
        }
    };
}

macro_rules! parse_mutability {
    ($pairs: tt, $pair: tt) => {
        match $pair.as_rule() {
            Rule::mutable => {
                let mutable = Mutability::Mutable;

                step_pair!($pairs, $pair);
                mutable
            }
            _ => Mutability::Immutable,
        }
    };
}

// macro_rules! parse_ident {
//     ($pairs: tt, $pair: tt) => {
//         match $pair.as_rule() {
//             Rule::ident => {
//                 let value = $pair.as_str();
//                 step_pair!($pairs, $pair);
//                 value.into()
//             }
//             _ => {
//                 panic!("Expected ident!");
//             }
//         }
//     };
// }
//
// macro_rules! parse_generic_params {
//     ($pairs: tt, $pair: tt) => {
//         match $pair.as_rule() {
//             Rule::type_generics_definition {
//                 let generics: Vec<String> = vec![];
//
//                 let inner_pairs = $pair.into_inner();
//                 for inner_pair in  {
//                     let value = parse
//                 }
//             },
//             _ => None
//         }
//     }
// }

macro_rules! expect_rule {
    // ($pair:ident, $rule:pat $(if $guard:expr)? $(,)?, $on_match:expr) => {
    ($pair:ident, $rule:pat, $on_match:expr) => {
        match $pair.as_rule() {
            $rule => Some($on_match),
            _ => None,
        }
    };
}

// fn expect_rule(pair: &Pair<'_, Rule>, expected_rule: Rule) -> Option<()> {
//     match pair.as_rule() {
//         rule if rule == expected_rule => Some(()),
//         _ => None,
//     }
// }

fn ignore_next_pair(pairs: &mut pest::iterators::Pairs<'_, Rule>) {
    pairs.next().unwrap();
}

fn parse_generic_parameters(pair: &Pair<'_, Rule>) -> Option<Box<[String]>> {
    expect_rule!(pair, Rule::type_generics_definition, {
        let mut params: Vec<String> = vec![];

        for inner_pair in pair.to_owned().into_inner() {
            let param = expect_rule!(inner_pair, Rule::ident, inner_pair.as_str().to_owned())
                .expect("Expected ident for generic parameter!");

            params.push(param);
        }

        params.into()
    })
}

fn parse_type(pair: &Pair<'_, Rule>) -> Type {
    expect_rule!(pair, Rule::r#type, {
        let mut inner_pairs = pair.to_owned().into_inner();

        let pair = inner_pairs.next().unwrap();

        let name = expect_rule!(pair, Rule::ident, pair.as_str()).expect("Expected a type!");

        match name {
            "int" => Type::Integer {
                signed: true,
                size: 64,
            },
            "float" => Type::Float { size: 64 },
            "bool" => Type::Boolean,
            _ => {
                let pair = inner_pairs.next();

                let generic_parameters = match pair {
                    Some(pair) => parse_generic_parameters(&pair),
                    None => None,
                };

                Type::UserDefined(UserDefinedType {
                    name: name.into(),
                    generic_parameters,
                })
            }
        }
    })
    .unwrap_or(Type::Inferred)
}

fn parse_function_parameter(pair: &Pair<'_, Rule>) -> Result<FunctionParameter, String> {
    expect_rule!(pair, Rule::function_parameter, {
        let mut inner_pairs = pair.to_owned().into_inner();
        let mut pair = inner_pairs.next().unwrap();

        let locality = parse_locality!(inner_pairs, pair);
        let exclusivity = parse_exclusivity!(inner_pairs, pair);
        let mutability = parse_mutability!(inner_pairs, pair);

        let name = expect_rule!(pair, Rule::ident, pair.as_str().into())
            .expect("Expected a name for the function parameter!");

        let pair = inner_pairs.next().unwrap();
        let r#type = parse_type(&pair);
        if matches!(r#type, Type::Inferred) {
            return Err("Expected a type for function parameter.".to_owned());
        }

        Ok(FunctionParameter {
            locality,
            exclusivity,
            mutability,
            name,
            r#type,
        })
    })
    .expect("Expected a function parameter!")
}

macro_rules! some_or {
    ($result:expr, $err_msg:expr) => {
        match $result {
            Some(r) => r,
            _ => {
                return Err($err_msg.to_owned());
            }
        }
    };
}

fn parse_function_definition(
    mut pairs: pest::iterators::Pairs<'_, Rule>,
) -> Result<FunctionDefinition, String> {
    let mut pair = pairs.next().unwrap();

    let visibility = parse_visibility!(pairs, pair);
    let locality = parse_locality!(pairs, pair);

    // Step over fn keyword
    step_pair!(pairs, pair);

    let name = some_or!(
        expect_rule!(pair, Rule::ident, pair.as_str().into()),
        format!("Expected ident for function name!\n{:#?}", pair)
    );

    step_pair!(pairs, pair);
    let generic_parameters = parse_generic_parameters(&pair);
    if generic_parameters.is_some() {
        pair = pairs.next().unwrap();
    }

    let parameters = expect_rule!(pair, Rule::function_parameters_list, {
        let inner_pair = pair.clone().into_inner();

        let mut params: Vec<FunctionParameter> = vec![];

        for param_pair in inner_pair {
            let param = parse_function_parameter(&param_pair)?;
            params.push(param);
        }

        params
    })
    .unwrap_or(vec![])
    .into();

    let return_type = parse_type(&pair);

    let body = BlockExpr {
        commands: vec![].into(),
    };

    Ok(FunctionDefinition {
        visibility,
        locality,
        name,
        generic_parameters,
        parameters,
        return_type,
        body,
    })
}

#[cfg(test)]
mod test {
    use snap_builder::build_snaps;

    use crate::*;

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

    fn ast_parsing(input: &str) -> String {
        let ast = parse(input);

        match ast {
            Ok(file) => format!("{:#?}", file),
            Err(e) => e,
        }
    }

    fn setup_trace() {
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

    #[test]
    fn test_fn_ast() {
        setup_trace();

        let contents = include_str!(concat!("../testdata/snapshots/functions.hi"));
        let mut settings = insta::Settings::clone_current();

        settings.set_snapshot_path("../testdata/output/asts/");
        settings.bind(|| {
            insta::assert_snapshot!(ast_parsing(contents));
        });
    }
}
