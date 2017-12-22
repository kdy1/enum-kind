#[macro_use]
extern crate enum_kind;

#[derive(Debug, Kind)]
#[kind(functions(is_a = "bool", prec = "u8"))]
pub enum Tokens {
    #[kind(is_a)]
    #[kind(prec = "7")]
    A,
    #[kind(prec = "6")] StructLike {},
    #[kind(prec = "5")] TupleLike(u8),

    #[kind(prec = "6")]
    #[cfg(feature = "not-used")]
    Unused,
}

#[test]
fn simple_bool() {
    assert!(Tokens::A.is_a());
    assert!(!Tokens::StructLike {}.is_a());
    assert!(!Tokens::TupleLike(5).is_a());
}

#[derive(Debug, Kind)]
#[kind(functions(is_left = "bool"))]
enum Either<A, B> {
    #[kind(is_left)] Left(A),
    Right(B),
}
