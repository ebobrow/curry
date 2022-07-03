use curry::curry;

fn main() {
    let res = add3(1)(2)(3);
    println!("{}", res);
}

#[curry]
fn add3(a: i32, b: i32, c: i32) -> i32 {
    a + b + c
}
