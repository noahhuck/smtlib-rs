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

#[test]
fn int_to_real_conversion() {
    let st = Storage::new();
    let int_val = Int::new(&st, 5);
    let real_val = Real::new(&st, 3.5);

    // Convert int to real using to_real function
    let int_as_real = int_val.to_real();

    // Now we can compare them
    let comparison = real_val.lt(int_as_real);
    println!("3.5 < 5.0 (converted): {}", comparison);

    // Test in a solver context
    let mut solver =
        Solver::new(&st, crate::backend::z3_binary::Z3Binary::new("z3").unwrap()).unwrap();
    solver.assert(comparison).unwrap();
    let result = solver.check_sat().unwrap();
    println!("Solver result: {:?}", result);
}

#[test]
fn real_int_conversions() {
    let st = Storage::new();

    // Test to_real conversion
    let int_val = Int::new(&st, 7);
    let real_from_int = int_val.to_real();

    // Test to_int conversion
    let real_val = Real::new(&st, 7.8);
    let int_from_real = real_val.to_int();

    // Test is_int predicate
    let is_integer = real_val.is_int();
    let real_integer = Real::new(&st, 5.0);
    let is_exact_integer = real_integer.is_int();

    println!("Int 7 to real: {}", real_from_int);
    println!("Real 7.8 to int: {}", int_from_real);
    println!("Is 7.8 an integer: {}", is_integer);
    println!("Is 5.0 an integer: {}", is_exact_integer);

    // Test mixed comparisons using conversions
    let comparison1 = real_val.gt(int_val.to_real()); // 7.8 > 7.0
    let comparison2 = real_val.to_int()._eq(int_val); // to_int(7.8) == 7

    println!("7.8 > 7: {}", comparison1);
    println!("to_int(7.8) == 7: {}", comparison2);
}
