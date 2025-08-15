use hyperon_atom::*;
use hyperon_macros::metta;
use hyperon_macros::print_stream;
use hyperon::metta::runner::number::*;
use hyperon::metta::runner::str::*;
use hyperon::metta::runner::bool::*;

#[test]
fn macros_metta_literal() {
    assert_eq!(metta!{1}, Atom::gnd(Number::Integer(1)));
    print_stream!{-1};
    assert_eq!(metta!{-1}, Atom::gnd(Number::Integer(-1)));
    assert_eq!(metta!{+1}, Atom::gnd(Number::Integer(1)));
    assert_eq!(metta!{1.0}, Atom::gnd(Number::Float(1.0)));
    assert_eq!(metta!{-1.0}, Atom::gnd(Number::Float(-1.0)));
    assert_eq!(metta!{+1.0}, Atom::gnd(Number::Float(1.0)));
    assert_eq!(metta!{"text"}, Atom::gnd(Str::from_str("text")));
    assert_eq!(metta!{True}, Atom::gnd(Bool(true)));
    assert_eq!(metta!{False}, Atom::gnd(Bool(false)));
}
