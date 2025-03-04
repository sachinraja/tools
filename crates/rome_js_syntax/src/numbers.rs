//! JS Number parsing.

use std::str::FromStr;

pub use num_bigint::BigInt;

fn split_into_radix_and_number(num: &str) -> (u32, String) {
    match num.get(0..2) {
        Some("0x") | Some("0X") => (16, num.get(2..).unwrap().replace('_', "")),
        Some("0b") | Some("0B") => (2, num.get(2..).unwrap().replace('_', "")),
        Some("0o") | Some("0O") => (8, num.get(2..).unwrap().replace('_', "")),
        _ => (10, num.replace('_', "")),
    }
}

/// Parse a js number as a string into a number.
pub fn parse_js_number(num: &str) -> Option<f64> {
    let (mut radix, raw) = split_into_radix_and_number(num);

    // account for legacy octal literals
    if raw.starts_with('0') && !raw.contains(&['8', '9']) {
        radix = 8
    }

    if radix == 10 {
        f64::from_str(&raw).ok()
    } else {
        i64::from_str_radix(&raw, radix).map(|num| num as f64).ok()
    }
}

/// Parse a big int number as a string into a number.
pub fn parse_js_big_int(num: &str) -> Option<BigInt> {
    let (radix, raw) = split_into_radix_and_number(num);

    let raw = if raw.get(raw.len() - 1..raw.len()) == Some("n") {
        raw.split_at(raw.len() - 1).0.to_string()
    } else {
        raw
    };
    BigInt::parse_bytes(raw.as_bytes(), radix)
}

#[cfg(test)]
mod tests {
    use crate::{
        AstNode, JsBigIntLiteralExpression, JsNumberLiteralExpression, SyntaxTreeBuilder,
        JS_BIG_INT_LITERAL, JS_BIG_INT_LITERAL_EXPRESSION, JS_NUMBER_LITERAL,
        JS_NUMBER_LITERAL_EXPRESSION,
    };
    use num_bigint::ToBigInt;

    fn assert_float(literal: &str, value: f64) {
        let mut tree_builder = SyntaxTreeBuilder::new();
        tree_builder.start_node(JS_NUMBER_LITERAL_EXPRESSION);
        tree_builder.token(JS_NUMBER_LITERAL, literal);
        tree_builder.finish_node();

        let node = tree_builder.finish();
        let number_literal = JsNumberLiteralExpression::cast(node).unwrap();
        assert_eq!(number_literal.as_number(), Some(value))
    }

    fn assert_bigint(literal: &str, value: u64) {
        let mut tree_builder = SyntaxTreeBuilder::new();
        tree_builder.start_node(JS_BIG_INT_LITERAL_EXPRESSION);
        tree_builder.token(JS_BIG_INT_LITERAL, literal);
        tree_builder.finish_node();

        let node = tree_builder.finish();
        let number_literal = JsBigIntLiteralExpression::cast(node).unwrap();
        assert_eq!(number_literal.as_number(), Some(value.to_bigint().unwrap()))
    }

    #[test]
    fn base_10_float() {
        assert_float("1234", 1234.0);
        assert_float("0", 0.0);
        assert_float("9e999", f64::INFINITY);
        assert_float("9e-999", 0.0);
    }

    #[test]
    fn base_16_float() {
        assert_float("0xFF", 255.0);
        assert_float("0XFF", 255.0);
        assert_float("0x0", 0.0);
        assert_float("0xABC", 2748.0);
        assert_float("0XABC", 2748.0);
    }

    #[test]
    fn base_2_float() {
        assert_float("0b0000", 0.0);
        assert_float("0B0000", 0.0);
        assert_float("0b11111111", 255.0);
        assert_float("0B11111111", 255.0);
    }

    #[test]
    fn base_8_float() {
        assert_float("0o77", 63.0);
        assert_float("0O77", 63.0);
        assert_float("0o0", 0.0);
        assert_float("0O0", 0.0);
    }

    #[test]
    fn base_8_legacy_float() {
        assert_float("051", 41.0);
        assert_float("058", 58.0);
    }

    #[test]
    fn base_10_bigint() {
        assert_bigint("1010n", 1010);
        assert_bigint("0n", 0);
        assert_bigint("9007199254740991n", 9007199254740991);
    }

    #[test]
    fn base_16_bigint() {
        assert_bigint("0xffn", 255);
        assert_bigint("0XFFn", 255);
        assert_bigint("0x1fffffffffffffn", 9007199254740991);
        assert_bigint("0X1fffffffffffffn", 9007199254740991);
    }

    #[test]
    fn base_2_bigint() {
        assert_bigint("0b0n", 0);
        assert_bigint("0B0n", 0);
        assert_bigint(
            "0b11111111111111111111111111111111111111111111111111111n",
            9007199254740991,
        );
        assert_bigint(
            "0B11111111111111111111111111111111111111111111111111111n",
            9007199254740991,
        );
    }
}
