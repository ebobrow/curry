// TODO: macro
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
}
