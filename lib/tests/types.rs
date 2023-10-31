use hyperon::*;
use hyperon::matcher::*;
use hyperon::metta::*;
use hyperon::metta::runner::arithmetics::*;
use hyperon::metta::interpreter::*;
use hyperon::space::grounding::GroundingSpace;

#[derive(Clone, PartialEq, Debug)]
pub struct IsInt{}

impl std::fmt::Display for IsInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IsInt")
    }
}

impl Grounded for IsInt {
    fn type_(&self) -> Atom {
        Atom::expr([ARROW_SYMBOL, ATOM_TYPE_ATOM, ATOM_TYPE_BOOL])
    }

    fn execute(&self, args: &[Atom]) -> Result<Vec<Atom>, ExecError> {
        let arg_error = || ExecError::from("IsInt expects single atom argument");
        let a = args.get(0).ok_or_else(arg_error)?;

        let is_int = match a.as_gnd::<Number>() {
            Some(Number::Integer(_)) => true,
            _ => false,
        };

        Ok(vec![Atom::gnd(Bool(is_int))])
    }

    fn match_(&self, other: &Atom) -> MatchResultIter {
        match_by_equality(self, other)
    }
}

#[test]
fn test_types_in_metta() {
    let mut space = GroundingSpace::new();
    space.add(expr!("=" ("check" (":" n "Int")) ({IsInt{}} n)));
    space.add(expr!("=" ("check" (":" n "Nat")) ("and" ("check" (":" n "Int")) ({GreaterOp{}} n {Number::Integer(0)}))));
    space.add(expr!("=" ("and" {Bool(true)} {Bool(true)}) {Bool(true)}));
    space.add(expr!("=" ("and" {Bool(true)} {Bool(false)}) {Bool(false)}));
    space.add(expr!("=" ("and" {Bool(false)} {Bool(true)}) {Bool(false)}));
    space.add(expr!("=" ("and" {Bool(false)} {Bool(false)}) {Bool(false)}));
    space.add(expr!("=" ("if" {Bool(true)} then else) then));
    space.add(expr!("=" ("if" {Bool(false)} then else) else));
    space.add(expr!(":" "if" ("->" "Bool" "Atom" "Atom" "Atom")));
    space.add(expr!("=" ("fac" n) ("if" ("check" (":" n "Nat")) ("if" ({EqualOp{}} n {Number::Integer(1)}) {Number::Integer(1)} ({MulOp{}} n ("fac" ({SubOp{}} n {Number::Integer(1)})))) ("Error"))));

    assert_eq!(interpret(&space, &expr!("check" (":" {Number::Integer(3)} "Int"))), Ok(vec![expr!({Bool(true)})]));
    assert_eq!(interpret(&space, &expr!("check" (":" {Number::Integer(-3)} "Int"))), Ok(vec![expr!({Bool(true)})]));
    assert_eq!(interpret(&space, &expr!("check" (":" {Number::Integer(3)} "Nat"))), Ok(vec![expr!({Bool(true)})]));
    assert_eq!(interpret(&space, &expr!("check" (":" {Number::Integer(-3)} "Nat"))), Ok(vec![expr!({Bool(false)})]));
    assert_eq!(interpret(&space, &expr!("if" ("check" (":" {Number::Integer(3)} "Nat")) "ok" "nok")), Ok(vec![expr!("ok")]));
    assert_eq!(interpret(&space, &expr!("if" ("check" (":" {Number::Integer(-3)} "Nat")) "ok" "nok")), Ok(vec![expr!("nok")]));
    assert_eq!(interpret(&space, &expr!("fac" {Number::Integer(1)})), Ok(vec![expr!({Number::Integer(1)})]));
    assert_eq!(interpret(&space, &expr!("fac" {Number::Integer(3)})), Ok(vec![expr!({Number::Integer(6)})]));
}
