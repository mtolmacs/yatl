extern crate proc_macro2;

/// Parse error to indicate in the macro body where the issue is
/// and what the precise issue which caused this failure
#[derive(Debug)]
pub struct Error {
  pub token: proc_macro2::TokenTree,
  pub msg: String,
}

pub type TokenId = usize;

#[derive(PartialEq, Debug)]
pub enum Token {
  Ident(String),
  Literal(Vec<u8>),
}

/// This is the "pointer" table which allow lookup of the next step
/// by the current char (u8).
pub enum LookupTable<'a, T> {
  Unknown,
  Incomplete(&'a [LookupTable<'a, T>; 2]),
  Complete(T),
}

/// This is the structure which the parser will use to
/// decode the char stream. The root [LookupTable]
/// is where the parser will start when the first character
/// is read from the source code. Then the parser steps
/// through the links of [LookupTable] elements until it decodes
/// a [Token], which rolls up to an expression etc.
pub struct Grammar<'a, T> {
  root: LookupTable<'a, T>,
}