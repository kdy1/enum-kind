# enum_kind


# Usage

```toml
[dependencies]
enum-kind = "0.1"
```

```rust
#[macro_use]
extern crate enum_kind;

#[derive(Kind)]
#[kind(functions(before_expr = "bool", starts_expr = "bool"))]
pub enum Tokens {
    #[kind(before_expr)]
    Semi,
    #[kind(starts_expr)]
    Lit,
    /// Used to invert bool
    #[kind(before_expr, starts_expr)]
    Bang,
}

#[derive(Kind)]
#[kind(functions(precedence = "u8"))]
pub enum BinOpToken {
    /// `==`
    #[kind(precedence = "6")]
    EqEq,
    /// `!=`
    #[kind(precedence = "6")]
    NotEq,
    /// `===`
    EqEqEq,
    /// `!==`
    NotEqEq,
    /// `<`
    #[kind(precedence = "7")]
    Lt,
    /// `<=`
    #[kind(precedence = "7")]
    LtEq,
    /// `>`
    Gt,
    #[kind(precedence = "7")]
    /// `>=`
    #[kind(precedence = "7")]
    GtEq,
    /// `<<`
    #[kind(precedence = "8")]
    LShift,
    /// `>>`
    #[kind(precedence = "8")]
    RShift,
    /// `>>>`
    #[kind(precedence = "8")]
    ZeroFillRShift,

    /// `+`
    #[kind(precedence = "9")]
    Plus,
    /// `-`
    #[kind(precedence = "9")]
    Minus,
    /// `*`
    #[kind(precedence = "10")]
    Mul,
    /// `/`
    #[kind(precedence = "10")]
    Div,
    /// `%`
    #[kind(precedence = "10")]
    Mod,

    /// `|`

    #[kind(precedence = "3")]
    BitOr,
    /// `^`
    #[kind(precedence = "4")]
    BitXor,
    /// `&`
    #[kind(precedence = "5")]
    BitAnd,

    /// `in`
    #[kind(precedence = "7")]
    In,
    /// `instanceof`
    #[kind(precedence = "7")]
    InstanceOf,

    /// `**`
    #[kind(precedence = "10")]
    Exp,

    /// `||`
    #[kind(precedence = "1")]
    LogicalOr,
    /// `&&`
    #[kind(precedence = "2")]
    LogicalAnd,
}

```