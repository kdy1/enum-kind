//! # Atributes on enum
//! ## functions
//! `#[kind(functions(name = "return_type"))]`
//!
//! ```rust
//! #[macro_use]
//! extern crate enum_kind;
//!
//! /// You can split attributes if you want.
//! #[derive(Kind)]
//! #[kind(functions(is_a = "bool", is_b = "bool"))]
//! #[kind(functions(is_a_or_b = "bool", num = "u8"))]
//! pub enum E {
//!     #[kind(is_a, is_a_or_b, num = "1")]
//!     A,
//!     /// You can split attributes if you want.
//!     #[kind(is_b)]
//!     #[kind(is_a_or_b)]
//!     #[kind(num = "2")]
//!     B(u8),
//!     /// Default value of bool is false if not specified and true if specified.
//!     ///
//!     /// Both struct like variant and tuple like variant are supported.
//!     #[kind(num = "3")]
//!     C {},
//! }
//! # fn main() {
//! assert!(E::A.is_a() && E::A.is_a_or_b() && !E::A.is_b());
//! assert_eq!(E::A.num(), 1);
//!
//! assert!(!E::B(0).is_a() && E::B(0).is_a_or_b() && E::B(0).is_b());
//! assert_eq!(E::B(0).num(), 2);
//!
//! assert!(!E::C{}.is_a() && !E::C{}.is_a_or_b() && !E::C{}.is_b());
//! assert_eq!(E::C{}.num(), 3);
//!

//! # }
//! ```

#[macro_use]
extern crate pmutil;
extern crate proc_macro2;
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;
#[macro_use]
extern crate synom;

use quote::ToTokens;

mod expand;
mod input;
mod parse;
mod util;

#[proc_macro_derive(Kind, attributes(kind))]
pub fn derive_kind(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse::<syn::DeriveInput>(input)
        .map(From::from)
        .expect("failed to parse derive input");
    let item = expand::expand(input);
    let tokens = item.into_tokens();

    println!("Expanded:{}", tokens);

    tokens.into()
}
