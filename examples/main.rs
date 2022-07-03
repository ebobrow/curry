use curry::{curry, partial};

fn main() {
    let _res = add3(1)(2)(3);
    let _pf = partial! { f 1 2 _ };
    let _pf = partial! { f _ 2 _ };
}

#[curry]
fn add3(a: i32, b: i32, c: i32) -> i32 {
    a + b + c
}

fn f(a: i32, b: i32, c: i32) -> i32 {
    a + b + c
}
