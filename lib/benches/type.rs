#![feature(test)]

extern crate test;

use test::Bencher;
use hyperon::*;
use hyperon::metta::types::*;
use hyperon::space::grounding::*;
use hyperon::metta::text::*;

fn metta_space(text: &str) -> GroundingSpace {
    let tokenizer = Tokenizer::new();
    let mut space = GroundingSpace::new();
    let mut parser = SExprParser::new(text);
    while let Some(atom) = parser.parse(&tokenizer).unwrap() {
        space.add(atom);
    }
    space
}

fn atom_with_depth(depth: usize) -> Atom {
    if depth == 0 {
        Atom::expr([Atom::sym("a"), Atom::sym("b")])
    } else {
        Atom::expr([atom_with_depth(depth - 1), atom_with_depth(depth - 1)])
    }
}

#[bench]
fn bench_get_atom_types_complex(bencher: &mut Bencher) {
    let space = metta_space("(: b B) (: b BB)");
    let atom = atom_with_depth(3);
    bencher.iter(|| {
        let types = get_atom_types_v2(&space, &atom);
        assert!(types.is_empty());
    })
}
