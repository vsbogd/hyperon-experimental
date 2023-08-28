use hyperon::*;
use hyperon::metta::runner::*;
use hyperon::metta::text::*;
use hyperon::metta::runner::arithmetics::*;

fn main() {
    let runner = new_metta_rust();
    let mut parser = SExprParser::new("
        (= (TupleCount $tuple) (if-equal $tuple () 0 (+ 1 (TupleCount (cdr $tuple)))))
        !(TupleCount (1 2 3 4 5 6 7 8 9 10 11 12))
    ");
    let result = runner.run(&mut parser);

    assert_eq!(result, Ok(vec![vec![expr!({Number::Integer(13)})]]));
}
