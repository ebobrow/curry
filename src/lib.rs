pub use macros::curry;
pub use macros::partial;

pub fn partial<A: Copy, B, C>(f: impl Fn(A, B) -> C, arg1: A) -> impl Fn(B) -> C {
    move |arg2| f(arg1, arg2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partial_works() {
        let f = |a, b| a + b;
        assert_eq!(f(2, 3), partial(f, 2)(3));
    }

    #[test]
    fn curry() {
        #[curry]
        fn add3(a: i32, b: i32, c: i32) -> i32 {
            a + b + c
        }
        assert_eq!(6, add3(1)(2)(3));
    }

    #[test]
    fn partial_macro() {
        let f = |a, b, c| a + b + c;
        assert_eq!(f(1, 2, 3), partial! { f _ 2 _ }(1, 3));
    }
}
