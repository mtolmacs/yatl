#![doc = include_str!("../README.md")]

extern crate yatl_parser;
extern crate yatl_generative;
extern crate proc_macro;

use proc_macro::TokenStream;


/// Generate grammar from the "formal" definition description
/// 
/// Rules you can use:
/// SOME_EXPR -> "=" OTHER_EXPR
/// OTHER_EXPR -> "0"
#[proc_macro]
pub fn generative(input: TokenStream) -> TokenStream {
  yatl_generative::generative(input.into()).into()
}

/// Generate the parser for the language
#[proc_macro]
pub fn parser(language: TokenStream) -> TokenStream {
  yatl_parser::parser(language.into()).into()
}