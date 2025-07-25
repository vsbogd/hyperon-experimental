//! Contains algorithms to walk through subexpressions of expression in
//! different ways.

use crate::*;

pub fn split_expr(expr: &Atom) -> Option<(&Atom, std::slice::Iter<Atom>)> {
    match expr {
        Atom::Expression(expr) => {
            let mut args = expr.children().iter();
            args.next().map_or(None, |op| Some((op, args)))
        },
        _ => None,
    }
}
