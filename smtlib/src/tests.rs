use terms::StaticSorted;

use super::*;
use crate::terms::{forall, Sorted};

#[test]
fn int_math() {
    let st = Storage::new();
    let x = Int::new_const(&st, "x");
    let y = Int::new_const(&st, "hello");
    // let x_named = x.labeled();
    let mut z = 12 + y * 4;
    z += 3;
    let w = x * x + z;
    println!("{w}");
}

#[test]
fn quantifiers() {
    let st = Storage::new();
    let x = Int::new_const(&st, "x");
    let y = Int::new_const(&st, "y");

    let res = forall(&st, (x, y), (x + 2)._eq(y));
    println!("{}", res.sterm());
}

#[test]
fn negative_numbers() {
    let st = Storage::new();
    let mut solver =
        Solver::new(&st, crate::backend::z3_binary::Z3Binary::new("z3").unwrap()).unwrap();
    let x = Int::new_const(&st, "x");
    solver.assert(x.lt(-1)).unwrap();
    let model = solver.check_sat_with_model().unwrap().expect_sat().unwrap();
    match model.eval(x) {
        Some(x) => println!("This is the value of x: {x}"),
        None => panic!("Oh no! This should never happen, as x was part of an assert"),
    }
}

#[test]
fn real_power() {
    let st = Storage::new();
    let mut solver =
        Solver::new(&st, crate::backend::z3_binary::Z3Binary::new("z3").unwrap()).unwrap();
    let x = Real::new_const(&st, "x");

    // Test: x^2 = 9, so x should be 3 or -3
    solver
        .assert(x.pow(Real::new(&st, 2.0))._eq(Real::new(&st, 9.0)))
        .unwrap();
    solver.assert(x.gt(Real::new(&st, 0.0))).unwrap(); // Force positive solution

    let model = solver.check_sat_with_model().unwrap().expect_sat().unwrap();
    match model.eval(x) {
        Some(val) => {
            println!("This is the value of x: {val}");
            // The result should be approximately 3.0
        }
        None => panic!("Oh no! This should never happen, as x was part of an assert"),
    }
}
