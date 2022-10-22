extern crate proc_macro;

mod grammar {
  pub mod grammar;
  pub mod generative;
}

mod parser;

use proc_macro::TokenStream;

/// Generate grammar from the "formal" definition description
/// 
/// Rules you can use:
/// SOME_EXPR -> "=" OTHER_EXPR
/// OTHER_EXPR -> "0"
#[proc_macro]
pub fn generative(input: TokenStream) -> TokenStream {
  grammar::generative::generative(input.into()).into()
}

/// Generate the parser for the language
#[proc_macro]
pub fn yatl(language: TokenStream) -> TokenStream {
  parser::yatl(language.into()).into()
}
