//! Parsing strings into [AST nodes][0] for [interpretation][1]
//!
//! Parsing methods return a tree of [AST nodes][0] from strings of the AnyDice language.
//! These trees can be evaluated using the [interpret module][1].
//!
//! [0]: super::ast
//! [1]: super::interpret

use lalrpop_util::lalrpop_mod;

use crate::ast;

pub mod error;
mod lexer;
mod token;
lalrpop_mod!(parser, "/parse/parser.rs");

/// If you are looking to evaluate many statements without interaction
pub fn parse_program(input: &str) -> Result<ast::Program, error::ParseError> {
    let lexer = lexer::Lexer::new(input);
    let parser = parser::ProgramParser::new();

    parser.parse(lexer).map_err(|err| err.into())
}

/// If you are looking to evaluate statements interactively
pub fn parse_statement(input: &str) -> Result<ast::Statement, error::ParseError> {
    let lexer = lexer::Lexer::new(input);
    let parser = parser::StatementParser::new();

    parser.parse(lexer).map_err(|err| err.into())
}

/// If you are looking to evaluate an expression without side efects
pub fn parse_expression(input: &str) -> Result<ast::Expression, error::ParseError> {
    let lexer = lexer::Lexer::new(input);
    let parser = parser::ExpressionParser::new();

    parser.parse(lexer).map_err(|err| err.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::functional::*;

    #[test]
    fn test_parse_strips_comments() {
        assert_eq!(
            parse_expression("\\ignore\\ 1 \\comments\\").unwrap(),
            integer(1)
        );
    }

    #[test]
    fn test_parse_parens() {
        assert_eq!(
            parse_expression("1 + (2 + 3)").unwrap(),
            add(
                Box::new(integer(1)),
                Box::new(add(Box::new(integer(2)), Box::new(integer(3)))),
            )
        );
    }

    #[test]
    fn test_parse_mismatch_parens_fail() {
        assert!(parse_expression("(1").is_err());
        assert!(parse_expression("1)").is_err());
        assert!(parse_expression("({1)}").is_err());
        assert!(parse_expression("{(1})").is_err());
    }

    #[test]
    fn test_parse_integer() {
        assert_eq!(parse_expression("1").unwrap(), integer(1));
    }

    #[test]
    fn test_parse_integer_overflow_fail() {
        assert!(parse_expression(&format!("{}", u32::MAX)).is_err());
    }

    #[test]
    fn test_parse_var_ref() {
        assert_eq!(
            parse_expression("FOO").unwrap(),
            variable_reference("FOO".to_owned())
        );

        assert_eq!(
            parse_expression("FOO_BAR").unwrap(),
            variable_reference("FOO_BAR".to_owned())
        );

        assert_eq!(
            parse_expression("_FOO_").unwrap(),
            variable_reference("_FOO_".to_owned())
        );

        assert_eq!(
            parse_expression("_").unwrap(),
            variable_reference("_".to_owned())
        );
    }

    #[test]
    fn test_parse_sequence() {
        assert_eq!(parse_expression("{}").unwrap(), sequence(vec![]));

        assert_eq!(
            parse_expression("{1}").unwrap(),
            sequence(vec![single_entry(integer(1), integer(1))])
        );

        assert_eq!(
            parse_expression("{1, 2:3}").unwrap(),
            sequence(vec![
                single_entry(integer(1), integer(1)),
                single_entry(integer(2), integer(3)),
            ])
        );

        assert_eq!(
            parse_expression("{1, 2:3, 4..5}").unwrap(),
            sequence(vec![
                single_entry(integer(1), integer(1)),
                single_entry(integer(2), integer(3)),
                range_entry(integer(4), integer(5), integer(1)),
            ])
        );

        assert_eq!(
            parse_expression("{1, 2:3, 4..5, 6..7:8}").unwrap(),
            sequence(vec![
                single_entry(integer(1), integer(1)),
                single_entry(integer(2), integer(3)),
                range_entry(integer(4), integer(5), integer(1)),
                range_entry(integer(6), integer(7), integer(8)),
            ])
        );
    }

    #[test]
    fn test_parse_mismatch_sequence_fail() {
        assert!(parse_expression("{1").is_err());
        assert!(parse_expression("1}").is_err());
        assert!(parse_expression("({1)}").is_err());
        assert!(parse_expression("{(1})").is_err());
    }

    #[test]
    fn test_parse_unary_operators() {
        assert_eq!(
            parse_expression("-!#1").unwrap(),
            negate(Box::new(not(Box::new(length(Box::new(integer(1))))))),
        );
    }

    #[test]
    fn test_parse_binary_operator_with_precedence() {
        assert_eq!(
            // All binary operations in reverse order
            // This is the expected order of operations highlighted with parens
            // (
            //     1 & 2 | (
            //         3 = 4 != 5 < 6 > 7 <= 8 >= (
            //             9 + 10 - (
            //                 11 * 12 / (
            //                     13 ^ (
            //                         14 @ (
            //                             15 d 16
            //                         )
            //                     )
            //                 )
            //             )
            //         )
            //     )
            // )
            parse_expression(
                "1 & 2 | 3 = 4 != 5 < 6 > 7 <= 8 >= 9 + 10 - 11 * 12 / 13 ^ 14 @ 15 d 16"
            )
            .unwrap(),
            or(
                Box::new(and(Box::new(integer(1)), Box::new(integer(2)))),
                Box::new(greater_or_equal(
                    Box::new(less_or_equal(
                        Box::new(greater(
                            Box::new(less(
                                Box::new(not_equal(
                                    Box::new(equal(Box::new(integer(3)), Box::new(integer(4)))),
                                    Box::new(integer(5)),
                                )),
                                Box::new(integer(6)),
                            )),
                            Box::new(integer(7)),
                        )),
                        Box::new(integer(8)),
                    )),
                    Box::new(subtract(
                        Box::new(add(Box::new(integer(9)), Box::new(integer(10)))),
                        Box::new(divide(
                            Box::new(multiply(Box::new(integer(11)), Box::new(integer(12)))),
                            Box::new(exponent(
                                Box::new(integer(13)),
                                Box::new(access(
                                    Box::new(integer(14)),
                                    Box::new(dice(Box::new(integer(15)), Box::new(integer(16)))),
                                )),
                            )),
                        )),
                    ))
                ))
            )
        );
    }

    #[test]
    fn test_parse_mismatch_paren_and_sequence_fail() {
        assert!(parse_expression("({1)}").is_err());
        assert!(parse_expression("{(1})").is_err());
    }
}
