use std::usize;

use crate::ql::lex::Token;

pub fn postfix_binding_power(_token: &Token) -> Option<(usize, usize)> {
    Some((1, 1))
}

pub fn prefix_binding_power(_token: &Token) -> Option<(usize, usize)> {
    Some((1, 1))
}

pub fn infix_binding_power(_token: &Token) -> Option<(usize, usize)> {
    Some((1, 1))
}
