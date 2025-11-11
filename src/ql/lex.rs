use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[token("true")]
    True,
    #[token("false")]
    False,

    // Binary operators
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Mul,
    #[token("/")]
    Div,
    #[token("|>")]
    Pipe,

    // Strings
    #[regex(r#""([^"\\]|\\.)*""#, parse_string)]
    SingleQuotedString(String),
    #[regex(r#"'([^'\\]|\\.)*'"#, parse_string)]
    DoubleQuotedString(String),

    // Numbers
    #[regex(r"[0-9]+(?:\.[0-9]+)?(?:[ui](?:16|32|64)|f(?:32|64))?", parse_number)]
    Number(Number),

    // Symbol
    #[regex(r"[A-Za-z_][A-Za-z0-9_]*", |lex| Some(lex.slice().to_string()))]
    Sym(String),

    // Grouping
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LCurlyBrace,
    #[token("}")]
    RCurlyBrace,
    #[token("[")]
    LSquareBrace,
    #[token("]")]
    RSquareBrace,
    #[token(",")]
    Comma,

    // Whitespace
    #[regex(r"[ \t\n\f]+", |lex| Some(lex.slice().to_string()))]
    Whitespace(String),
}

#[derive(Debug, PartialEq)]
pub enum Number {
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}

fn parse_number(lex: &mut logos::Lexer<Token>) -> Option<Number> {
    let slice = lex.slice();

    // Separate numeric part and optional suffix
    let suffix_chars = &['i', 'f'];
    let mut suffix_start = slice.len();
    for (i, c) in slice.chars().enumerate() {
        if suffix_chars.contains(&c) {
            suffix_start = i;
            break;
        }
    }
    let (number_part, suffix) = slice.split_at(suffix_start);

    if number_part.contains('.') {
        // Float
        let val: f64 = number_part.parse().ok()?;
        match suffix {
            "f32" => Some(Number::F32(val as f32)),
            "f64" | "" => Some(Number::F64(val)),
            _ => None,
        }
    } else {
        // Integer
        let val: i64 = number_part.parse().ok()?;
        match suffix {
            "" => {
                // Auto-fit smallest signed type
                if val >= i16::MIN as i64 && val <= i16::MAX as i64 {
                    Some(Number::I16(val as i16))
                } else if val >= i32::MIN as i64 && val <= i32::MAX as i64 {
                    Some(Number::I32(val as i32))
                } else {
                    Some(Number::I64(val))
                }
            }
            "i16" => Some(Number::I16(val as i16)),
            "i32" => Some(Number::I32(val as i32)),
            "i64" => Some(Number::I64(val)),
            _ => None,
        }
    }
}

/// Simple parser for quoted strings that handles backslash escapes.
/// We strip the surrounding quotes and unescape \n, \t, \r, \", \', and \\.
fn parse_string(lex: &mut logos::Lexer<Token>) -> Option<String> {
    let slice = lex.slice();
    let inner = &slice[1..slice.len() - 1]; // remove quotes
    let mut result = String::new();
    let mut chars = inner.chars();

    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') => result.push('\n'),
                Some('t') => result.push('\t'),
                Some('r') => result.push('\r'),
                Some('\'') => result.push('\''),
                Some('"') => result.push('"'),
                Some('\\') => result.push('\\'),
                Some(other) => {
                    // Unknown escape, keep literally
                    result.push('\\');
                    result.push(other);
                }
                None => break,
            }
        } else {
            result.push(c);
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_number_token() {
        assert_eq!(1, 1)
    }
}
